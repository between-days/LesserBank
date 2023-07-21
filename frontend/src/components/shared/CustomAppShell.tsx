import {
    AppShell,
    Header,
    MediaQuery,
    Burger,
    useMantineTheme,
    Title,
    ThemeIcon,
    Center,
    Text,

} from '@mantine/core';

import React from 'react';
import { CustomNavbar } from './CustomNavBar';
import { IconMathLower } from '@tabler/icons-react';

interface CustomAppShellProps {
    title: string
    children: any
}

export default function CustomAppShell({ title, children }: CustomAppShellProps) {
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
                    <div style={{ display: 'flex', alignItems: 'center', justifyContent: "space-between", height: '100%' }}>
                        <Text fz="xl" fw={300} >{title}</Text>

                        <Center>
                            <ThemeIcon radius='md' variant="outline" size="lg">
                                <IconMathLower />
                            </ThemeIcon>
                        </Center>
                    </div>
                </Header>
            }
        >
            {children}
        </AppShell>
    );
}