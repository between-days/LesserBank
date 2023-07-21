import { useState } from 'react';
import { createStyles, Navbar, Title, rem, Space, ThemeIcon, Group, Stack } from '@mantine/core';
import {
    IconHome2,
    IconMathLower,
} from '@tabler/icons-react';

const useStyles = createStyles((theme) => ({
    wrapper: {
        display: 'flex',
    },

    main: {
        flex: 1,
        backgroundColor: theme.colorScheme === 'dark' ? theme.colors.dark[6] : theme.colors.gray[0],
    },

    title: {
        boxSizing: 'border-box',
        fontFamily: `Greycliff CF, ${theme.fontFamily}`,
        marginBottom: theme.spacing.xl,
        backgroundColor: theme.colorScheme === 'dark' ? theme.colors.dark[7] : theme.white,
        padding: theme.spacing.md,
        paddingTop: rem(18),
        height: rem(60),
        borderBottom: `${rem(1)} solid ${theme.colorScheme === 'dark' ? theme.colors.dark[7] : theme.colors.gray[3]
            }`,
    },

    logo: {
        boxSizing: 'border-box',
        width: '100%',
        display: 'flex',
        justifyContent: 'center',
        height: rem(60),
        paddingTop: theme.spacing.md,
        borderBottom: `${rem(1)} solid ${theme.colorScheme === 'dark' ? theme.colors.dark[7] : theme.colors.gray[3]
            }`,
        marginBottom: theme.spacing.xl,
    },

    link: {
        boxSizing: 'border-box',
        display: 'block',
        textDecoration: 'none',
        borderTopRightRadius: theme.radius.md,
        borderBottomRightRadius: theme.radius.md,
        color: theme.colorScheme === 'dark' ? theme.colors.dark[0] : theme.colors.gray[7],
        padding: `0 ${theme.spacing.md}`,
        fontSize: theme.fontSizes.sm,
        fontWeight: 500,
        lineHeight: rem(44),

        '&:hover': {
            backgroundColor: theme.colorScheme === 'dark' ? theme.colors.dark[5] : theme.colors.gray[1],
            color: theme.colorScheme === 'dark' ? theme.white : theme.black,
        },
    },

    linkActive: {
        '&, &:hover': {
            borderLeftColor: theme.fn.variant({ variant: 'filled', color: theme.primaryColor })
                .background,
            backgroundColor: theme.fn.variant({ variant: 'filled', color: theme.primaryColor })
                .background,
            color: theme.white,
        },
    },
}));

const linksMockdata = [
    'Dashboard',
    "Transaction"
];

export function CustomNavBarAlt2() {
    const { classes, cx } = useStyles();
    const [activeLink, setActiveLink] = useState('Settings');

    const links = linksMockdata.map((link) => (
        <Group  >
            <ThemeIcon variant="light" size="xl">
                <IconHome2 />
            </ThemeIcon>
            <a
                className={cx(classes.link, { [classes.linkActive]: activeLink === link })}
                href="/"
                onClick={(event) => {
                    setActiveLink(link);
                }}
                key={link}
            >
                <Title order={3}>
                    {link}
                </Title>
            </a>
        </Group>
    ));

    return (
        <Navbar width={{ sm: 300 }}>
            <Navbar.Section grow className={classes.wrapper}>
                <div className={classes.main}>
                    <Space h="xl" />
                    <Space h="xl" />
                    <ThemeIcon>
                        <IconMathLower />
                    </ThemeIcon>
                    <Space h="xl" />
                    <Stack>
                        {links}
                    </Stack>
                </div>
            </Navbar.Section>
        </Navbar>
    );
}