import { Navbar, Space, Group, Stack, NavLink, Text, Avatar } from '@mantine/core';
import {
    IconActivity,
    IconHome2,
    IconArrowBarToRight
} from '@tabler/icons-react';
import { useRouter } from 'next/router';

const pages = [
    {
        icon: IconHome2, label: 'Dashboard',
        description: 'View your accounts',
        route: '/dashboard'
    },
    {
        icon: IconArrowBarToRight,
        label: 'Transactions',
        description: 'View all transactions', route: '/'
    },
    {
        icon: IconActivity,
        label: 'Activity',
        route: '/'
    },
];

export function CustomNavBar() {
    const router = useRouter()
    const activeRoute = router.asPath

    const items = pages.map((item, index) => (
        <NavLink
            key={item.label}
            active={item.route == activeRoute}
            label={<Text fw={500}>
                {item.label}
            </Text>}
            description={item.description}
            icon={<item.icon size="2rem" stroke={1.5} />}
            onClick={() => router.push(item.route)}
        />
    ));

    return (
        <Navbar width={{ sm: 250 }}>
            <Navbar.Section grow >
                <Stack mt="0.5rem" >
                    <Group align='center' p="md">
                        <Avatar src="bingus.jpg" alt="uh oh" radius="xl" size="lg" />
                        <Text fw={250} fz="lg">
                            Welcome, Bingus
                        </Text>
                    </Group>
                    <Space h="xl" />
                    {items}
                </Stack>
            </Navbar.Section>
        </Navbar>
    );
}