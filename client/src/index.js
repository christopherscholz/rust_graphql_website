import React from 'react';
import ReactDOM from 'react-dom/client';
import './css/normalize.css';
import './css/index.css';
import { BrowserRouter, Routes, Route } from "react-router-dom";
import Page from "./pages/Page";
import NoPage from "./pages/NoPage";
import { ApolloClient, InMemoryCache, ApolloProvider } from '@apollo/client';

const root = ReactDOM.createRoot(document.getElementById('root'));

const client = new ApolloClient({
    uri: 'http://127.0.0.1:8000/graphql',
    cache: new InMemoryCache(),
});

root.render(
    <React.StrictMode>
        <ApolloProvider client={client}>
            <BrowserRouter>
                <Routes>
                    <Route index element={<Page page="home" />} />
                    <Route path="impressum" element={<Page page="impressum" />} />
                    <Route path="*" element={<NoPage />} />
                </Routes>
            </BrowserRouter>
        </ApolloProvider>
    </React.StrictMode>
);
