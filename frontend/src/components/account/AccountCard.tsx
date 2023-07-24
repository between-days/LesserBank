import { Card, Text, Group, Stack, MantineShadow, Center, Button } from '@mantine/core';
import { useHover } from '@mantine/hooks';
import { useRouter } from 'next/router';
import { getAccountNumberString, getBsbString, getDollarTextFromCents, getIconForAccountType } from '@/UIUtils';
import { AccountType } from '@/interfaces';

export interface AccountCardProps {
    name: string | undefined
    accountType: AccountType
    balanceCents: number
    availableBalanceCents: number
    accountNumber: string
    bsb: number,
    onHover: boolean
}

export default function AccountCard({ name, accountType, balanceCents, accountNumber, availableBalanceCents, bsb, onHover }: AccountCardProps) {
    const router = useRouter()
    const { hovered, ref } = useHover();

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
            <Stack spacing="xl">
                <Group position='apart' mt='lg'>
                    <Group position='left' noWrap>
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