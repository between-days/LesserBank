import { Card, Text, Group, Stack, Flex, MantineShadow, useMantineTheme, Tooltip, Space, Center, Button } from '@mantine/core';
import { useHover } from '@mantine/hooks';
import { useRouter } from 'next/router';
import { getAccountNumberString, getBsbString, getDollarTextFromCents, getIconForAccountType } from '@/UIUtils';
import { AccountType } from '@/interfaces';

export interface AccountCardAltProps {
    name: string | undefined
    accountType: AccountType
    balanceCents: number
    availableBalanceCents: number
    accountNumber: number
    bsb: number,
    onHover: boolean
}

export default function AccountCardAlt({ name, accountType, balanceCents, accountNumber, availableBalanceCents, bsb, onHover }: AccountCardAltProps) {
    const router = useRouter()
    const { hovered, ref } = useHover();
    const theme = useMantineTheme();

    let shadow: MantineShadow | undefined = "sm"
    let withBorder: boolean = false
    if (onHover) {
        shadow = hovered ? "xl" : "sm"
        withBorder = hovered ? false : true
    }

    if (!name || name.length == 0) {
        name = accountType.toUpperCase() + " ACCOUNT"
    }

    const onClick = () => {
        router.push(`/accounts/${accountNumber}`)
    }

    return (
        <Card ref={ref} shadow={shadow} radius="lg" withBorder={withBorder}>
            <Card.Section >
                <Center
                    sx={(theme) => ({
                        height: '0.5rem',
                        backgroundImage: theme.fn.gradient({ from: 'red', to: 'orange', deg: 45 }),
                        color: theme.white,
                    })}
                >
                </Center>
            </Card.Section>

            <Stack spacing="xl">
                <Group position='apart' mt='lg'>
                    <Group position='left'>
                        {getIconForAccountType(accountType)}
                        <Stack spacing={0}>
                            <Text weight={500} fz="lg">{name}</Text>
                            <Group>
                                <Text color="dimmed" fz="sm">
                                    {getAccountNumberString(accountNumber)}
                                </Text>
                                <Text color="dimmed" fz="sm">
                                    {getBsbString(bsb)}
                                </Text>
                            </Group>
                        </Stack>
                    </Group>
                </Group>
                <Group position='apart'>
                    <Text weight={500} fz="lg">
                        {getDollarTextFromCents(availableBalanceCents)}
                    </Text>
                    <Button variant="light" onClick={onClick}>
                        Details
                    </Button>
                </Group>
            </Stack>
        </Card>
    );
}