import { Card, Image, Text, Group, Tooltip, Stack, Flex, ThemeIcon } from '@mantine/core';
import { useHover } from '@mantine/hooks';
import { IconCloudDollar, IconTimeDuration0 } from '@tabler/icons-react';

export interface AccountCardProps {
    name: string
    accountType: string
    balanceCents: number
    accountNumber: number
    bsb: number
}

const ACCOUNT_ICON_SIZE = "xl"

function getIconForAccountType(accountType: string) {
    switch (accountType) {
        case "Savings":
            return <Tooltip label={"Savings"} position="right" transitionProps={{ duration: 0 }}>
                <ThemeIcon size={ACCOUNT_ICON_SIZE}>
                    <IconCloudDollar />
                </ThemeIcon>

            </Tooltip>
        case "Term Deposit":
            return <Tooltip label={"Term Deposit"} position="right" transitionProps={{ duration: 0 }}>
                <ThemeIcon size={ACCOUNT_ICON_SIZE}>
                    <IconTimeDuration0 />
                </ThemeIcon>

            </Tooltip>
        default:
            return <Tooltip label={"Unkown Type"} position="right" transitionProps={{ duration: 0 }}>
                <ThemeIcon size={ACCOUNT_ICON_SIZE}>
                    <IconCloudDollar />
                </ThemeIcon>
            </Tooltip>
    }
}

function getBalanceText(balanceCents: number) {
    const balanceDollars = balanceCents / 100
    return `$${balanceDollars.toLocaleString()}`
}

function getBsbString(bsb: number) {
    const bsbString = bsb.toString()
    return bsbString.substring(0, 3) + " " + bsbString.substring(3, 6)
}

function getAccountNumberString(accountNumber: number) {
    const anString = accountNumber.toString()
    return anString.substring(0, 3) + " " + anString.substring(3, 6) + " " + anString.substring(6, 10)
}

function getCardImage(accountType: string) {
    switch (accountType) {
        case "Savings":
            return <Image
                src="https://images.unsplash.com/photo-1579621970588-a35d0e7ab9b6?ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D&auto=format&fit=crop&w=3540&q=80"
                height={150}
                alt="Norway"
            />
        case "Term Deposit":
            return <Image
                src="https://images.unsplash.com/photo-1531161348856-92e5c7b2e6de?ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D&auto=format&fit=crop&w=3542&q=80"
                height={150}
                alt="Norway"
            />
        default:
            return <Image
                src="https://images.unsplash.com/photo-1579621970588-a35d0e7ab9b6?ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D&auto=format&fit=crop&w=3540&q=80"
                height={150}
                alt="Norway"
            />
    }
}

export default function AccountCard({ name, accountType, balanceCents, accountNumber, bsb }: AccountCardProps) {
    const { hovered, ref } = useHover();

    return (
        <Card ref={ref} shadow={hovered ? "xl" : "sm"} radius="md" withBorder={hovered ? false : true}>
            <Card.Section>
                {getCardImage(accountType)}
            </Card.Section>

            <Group position="apart" mt="md" mb="xs">
                <Text weight={500}>{name}</Text>
                {getIconForAccountType(accountType)}
            </Group>

            <Stack spacing="xs">
                <Text color="green" weight={500} fz="xl" >Balance: {getBalanceText(balanceCents)}</Text>
                <Flex
                    mih={50}
                    gap="xl"
                    justify="space-between"
                    align="center"
                    direction="row"
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