import { AppProps } from 'next/app';
import Head from 'next/head';
import { MantineProvider } from '@mantine/core';
import { rtlCache } from '../rtl-cache'

export default function App(props: AppProps) {
    const { Component, pageProps } = props;
    return (
        <>
            <Head>
                <title>Page title</title>
                <meta name="viewport" content="minimum-scale=1, initial-scale=1, width=device-width" />
            </Head>
            <MantineProvider
                withGlobalStyles
                withNormalizeCSS
                emotionCache={rtlCache}
                theme={
                    {
                        colors: {
                            dark: [
                                "#f9f9fb",
                                "#eeecf3",
                                "#e3e0eb",
                                "#d8d3e4",
                                "#3c3352",
                                "#322b45",
                                "#292339",
                                "#201b2c",
                                "#0e0c13",
                                "#050406",
                            ],
                            flame: [
                                "#f8dfd3",
                                "#f8dfd3",
                                "#e38759",
                                "#e38759",
                                "#e07a47",
                                "#e07a47",
                                "#dd6e36",
                                "#dd6e36",
                                "#da6125",
                                "#da6125"
                            ]
                        },
                        colorScheme: 'light',
                        primaryColor: "flame"
                    }
                }
            >
                <Component {...pageProps} />
            </MantineProvider>
        </>
    );
}