import React from 'react';

import { useQuery, gql } from '@apollo/client';

function Page(props){
    
    const GET_PAGE = gql`
        query GetPage {
            page(name: "${props.page}") {blocks {
                id
                type
                ... on ParagraphBlock{data{text}}
                ... on HeaderBlock{data{text,level}}
                ... on ListBlock{data{style,items}}
            }}
        }
    `;

    const { loading, error, data } = useQuery(GET_PAGE);
    if (loading) return <main>Loading...</main>;
    if (error) return <main>Error</main>;

    return (
        <main>
            { data.page.blocks.map(((block, index) => {
                switch (block.type) {
                    case 'paragraph':
                        return <p dangerouslySetInnerHTML={{ __html: block.data.text }} />;
                    case 'header':
                        const HeadingTag = `h${block.data.level}`;
                        return <HeadingTag dangerouslySetInnerHTML={{ __html: block.data.text }} />;
                    case 'list':
                        let ListTag;
                        let ItemTag;
                        if (block.style === 'ordered') {
                            ListTag = `ol`;
                            ItemTag = 'li';
                        } else {
                            ListTag = `ul`;
                            ItemTag = 'li';
                        }

                        return (
                            <ListTag>
                                {block.data.items.map((item, index) =>
                                    <ItemTag dangerouslySetInnerHTML={{ __html: item }} />
                                )}
                            </ListTag>
                        );
                    default:
                        return <p>Block is not supported</p>;
                }
            }))}
        </main>
    );
};
  
export default Page;