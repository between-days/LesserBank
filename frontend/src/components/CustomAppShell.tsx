import {
    AppShell,
    Header,
    MediaQuery,
    Burger,
    useMantineTheme,
    Title,

} from '@mantine/core';

import React from 'react';
import { CustomNavbar } from './CustomNavBar';

export default function CustomAppShell(props: any) {
    const theme = useMantineTheme();

    return (
        <AppShell
            styles={{
                main: {
                    background: theme.colorScheme === 'dark' ? theme.colors.dark[8] : theme.colors.gray[0],
                },
            }}
            navbarOffsetBreakpoint="sm"

            layout="default"
            fixed={true}

            navbar={
                <CustomNavbar />
            }

            header={
                <Header height={{ base: 50, md: 70 }} p="md">
                    <div style={{ display: 'flex', alignItems: 'center', height: '100%' }}>
                        <MediaQuery largerThan="sm" styles={{ display: 'none' }}>
                            <Burger
                                opened={true}

                                size="sm"
                                color={theme.colors.gray[6]}
                                mr="xl"
                            />
                        </MediaQuery>
                        <Title>Accounts</Title>
                    </div>
                </Header>
            }
        >
            {props.children}
        </AppShell>
    );
}