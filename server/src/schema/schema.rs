use std::{collections::HashMap};

use juniper::{graphql_interface, graphql_object, Context, GraphQLEnum};
use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct Query;

#[graphql_object(context = Database)]
impl Query {
    fn page(
        #[graphql(context)] database: &Database,
        #[graphql(description = "name of the page")] name: String,
    ) -> Option<&Page> {
        database.get_page(&name)
    }
}

#[derive(Clone)]
pub struct Page {
    name: String,
    time: DateTime<Utc>,
    blocks: Vec<BlockValue>,
    version: String,
}

impl Page {
    pub fn new(
        name: &str,
        time: DateTime<Utc>,
        blocks: &[BlockValue],
        version: &str,
    ) -> Self {
        Self {
            name: name.into(),
            time: time.into(),
            blocks: blocks.to_vec(),
            version: version.into(),
        }
    }
}

#[graphql_object()]
impl Page {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn time(&self) -> &DateTime<Utc> {
        &self.time
    }

    pub fn blocks(&self) -> &Vec<BlockValue> {
        &self.blocks
    }

    pub fn version(&self) -> &str {
        &self.version
    }
}

#[graphql_interface(for = [ParagraphBlock, HeaderBlock, ListBlock])]
pub trait Block {
    fn id(&self) -> &Uuid;

    #[graphql(name = "type")]
    fn block_type(&self) -> &str;
}

impl Clone for BlockValue {
    #[inline]
    fn clone(&self) -> Self {
        match self {
            Self::ParagraphBlock(p) => Self::ParagraphBlock(p.clone()),
            Self::HeaderBlock(h) => Self::HeaderBlock(h.clone()),
            Self::ListBlock(l) => Self::ListBlock(l.clone()),
        }
    }
}

#[derive(Clone)]
pub struct ParagraphBlock {
    id: Uuid,
    data: ParagraphData,
}

impl ParagraphBlock {
    pub fn new(
        id: Uuid,
        data: ParagraphData,
    ) -> Self {
        Self {
            id: id,
            data: data,
        }
    }
}

#[graphql_object(impl = BlockValue)]
impl ParagraphBlock {
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    #[graphql(name = "type")]
    pub fn block_type(&self) -> &str {
        "paragraph"
    }

    pub fn data(&self) -> &ParagraphData {
        &self.data
    }
}

#[graphql_interface]
impl Block for ParagraphBlock {
    fn id(&self) -> &Uuid {
        &self.id
    }

    fn block_type(&self) -> &str {
        "paragraph"
    }
}

#[derive(Clone)]
pub struct HeaderBlock {
    id: Uuid,
    data: HeaderData,
}

impl HeaderBlock {
    pub fn new(
        id: Uuid,
        data: HeaderData,
    ) -> Self {
        Self {
            id: id,
            data: data,
        }
    }
}

#[graphql_interface]
impl Block for HeaderBlock {
    fn id(&self) -> &Uuid {
        &self.id
    }

    fn block_type(&self) -> &str {
        "header"
    }
}

#[graphql_object(impl = BlockValue)]
impl HeaderBlock {
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    #[graphql(name = "type")]
    pub fn block_type(&self) -> &str {
        "header"
    }

    pub fn data(&self) -> &HeaderData {
        &self.data
    }
}

#[derive(Clone)]
pub struct ListBlock {
    id: Uuid,
    data: ListData,
}

impl ListBlock {
    pub fn new(
        id: Uuid,
        data: ListData,
    ) -> Self {
        Self {
            id: id,
            data: data,
        }
    }
}

#[graphql_interface]
impl Block for ListBlock {
    fn id(&self) -> &Uuid {
        &self.id
    }

    fn block_type(&self) -> &str {
        "list"
    }
}

#[graphql_object(impl = BlockValue)]
impl ListBlock {
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    #[graphql(name = "type")]
    pub fn block_type(&self) -> &str {
        "list"
    }

    pub fn data(&self) -> &ListData {
        &self.data
    }
}

#[derive(Clone)]
pub struct ParagraphData {
    text: String,
}

impl ParagraphData {
    pub fn new(
        text: &str,
    ) -> Self {
        Self {
            text: text.into(),
        }
    }
}

#[graphql_object()]
impl ParagraphData {
    pub fn text(&self) -> &str {
        &self.text
    }
}

#[derive(Clone)]
pub struct HeaderData {
    text: String,
    level: i32,
}

impl HeaderData {
    pub fn new(
        text: &str,
        level: i32,
    ) -> Self {
        Self {
            text: text.into(),
            level: level.into(),
        }
    }
}

#[graphql_object()]
impl HeaderData {
    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn level(&self) -> &i32 {
        &self.level
    }
}


#[derive(GraphQLEnum, Clone, Copy, Debug, Eq, PartialEq)]
pub enum ListStyle {
    UNORDERED,
    ORDERED,
}

#[derive(Clone)]
pub struct ListData {
    style: ListStyle,
    items: Vec<String>,
}

impl ListData {
    pub fn new(
        style: ListStyle,
        items: &[&str],
    ) -> Self {
        Self {
            style: style,
            items: items.iter().copied().map(Into::into).collect(),
        }
    }
}

#[graphql_object()]
impl ListData {
    pub fn style(&self) -> &ListStyle {
        &self.style
    }

    pub fn items(&self) -> &Vec<String> {
        &self.items
    }
}

#[derive(Clone, Default)]
pub struct Database {
    pages: HashMap<String, Page>
}

impl Context for Database {}

impl Database {
    pub fn new() -> Database {
        let mut pages = HashMap::new();
        
        pages.insert(
            "home".into(),
            Page::new(
                "home",
                Utc::now(),
                &[
                    BlockValue::HeaderBlock(HeaderBlock::new(
                        Uuid::new_v4(),
                        HeaderData::new(
                            "Data Engineer, Problem Solver",
                            2
                        )
                    )),
                    BlockValue::ParagraphBlock(ParagraphBlock::new(
                        Uuid::new_v4(),
                        ParagraphData::new(
                            "<span class=\"highlight\">Data and processes accompany me through my entire professional life. As an expert in data and processes, especially in supply chain management, production and their interfaces, who speaks both the technical and the business language and can interpret in between, I contribute strongly to the understanding and better communication of problems.</span>"
                        )
                    ))
                ],
                "0.1.0",
            ),
        );

        pages.insert(
            "impressum".into(),
            Page::new(
                "impressum",
                Utc::now(),
                &[
                    BlockValue::HeaderBlock(HeaderBlock::new(
                        Uuid::new_v4(),
                        HeaderData::new(
                            "Angaben gemäß §5 TMG",
                            2
                        )
                    )),
                    BlockValue::ParagraphBlock(ParagraphBlock::new(
                        Uuid::new_v4(),
                        ParagraphData::new(
                            "Christopher Scholz<br>An der Dahme 3<br>12527 Berlin"
                        )
                    )),
                    BlockValue::HeaderBlock(HeaderBlock::new(
                        Uuid::new_v4(),
                        HeaderData::new(
                            "Kontakt",
                            2
                        )
                    )),
                    BlockValue::ParagraphBlock(ParagraphBlock::new(
                        Uuid::new_v4(),
                        ParagraphData::new(
                            "Email: <a href=\"mailto:website@christopher-scholz.com\">website@christopher-scholz.com</a>"
                        )
                    ))
                ],
                "0.1.0",
            ),
        );

        Database { pages }
    }

    pub fn get_page(&self, page: &str) -> Option<&Page> {
        self.pages.get(page)
    }
}