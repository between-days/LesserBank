import { Card, Text, Group, Stack, Flex, MantineShadow, useMantineTheme, Tooltip, Space, Center } from '@mantine/core';
import { useHover } from '@mantine/hooks';
import { useRouter } from 'next/router';
import { getAccountNumberString, getBsbString, getDollarTextFromCents, getIconForAccountType } from '@/UIUtils';
import { AccountType } from '@/interfaces';

export interface AccountCardProps {
    name: string | undefined
    accountType: AccountType
    balanceCents: number
    availableBalanceCents: number
    accountNumber: number
    bsb: number,
    onHover: boolean
}

export default function AccountCard({ name, accountType, balanceCents, accountNumber, availableBalanceCents, bsb, onHover }: AccountCardProps) {
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

    return (
        <Card ref={ref} onClick={() => { router.push(`/accounts/${accountNumber}`) }} shadow={shadow} radius="lg" withBorder={withBorder}>
            <Card.Section >
                <Center
                    sx={(theme) => ({
                        height: '1rem',
                        backgroundImage: theme.fn.gradient({ from: 'red', to: 'orange', deg: 45 }),
                        color: theme.white,
                    })}
                >
                </Center>
            </Card.Section>

            <Group position="apart" mt="md" mb="xs">
                <Text weight={500}>{name}</Text>
                {getIconForAccountType(accountType)}
            </Group>

            <Stack spacing="xs">
                <Text weight={500} fz="lg">
                    {getDollarTextFromCents(balanceCents)}
                </Text>
                <Flex
                    mih={50}
                    justify="space-between"
                    align="center"
                    wrap="wrap"
                >
                    <Text color="dimmed">
                        {getAccountNumberString(accountNumber)}
                    </Text>
                    <Text color="dimmed">
                        {getBsbString(bsb)}
                    </Text>
                </Flex>
            </Stack>
        </Card>
    );
}