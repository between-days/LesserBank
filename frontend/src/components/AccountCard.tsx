import { Card, Text, Group, Stack, Flex, MantineShadow } from '@mantine/core';
import { useHover } from '@mantine/hooks';
import { useRouter } from 'next/router';
import { getAccountCardImage, getAccountNumberString, getBsbString, getDollarText, getIconForAccountType } from '@/UIUtils';

export interface AccountCardProps {
    name: string
    accountType: string
    balanceCents: number
    accountNumber: number
    bsb: number,
    onHover: boolean
}

export default function AccountCard({ name, accountType, balanceCents, accountNumber, bsb, onHover }: AccountCardProps) {
    const router = useRouter()
    const { hovered, ref } = useHover();

    let shadow: MantineShadow | undefined = "sm"
    let withBorder: boolean = false
    if (onHover) {
        shadow = hovered ? "xl" : "sm"
        withBorder = hovered ? false : true
    }

    return (
        <Card ref={ref} onClick={() => { router.push(`/accounts/${accountNumber}`) }} shadow={shadow} radius="md" withBorder={withBorder}>
            <Card.Section>
                {getAccountCardImage(accountType)}
            </Card.Section>

            <Group position="apart" mt="md" mb="xs">
                <Text weight={500}>{name}</Text>
                {getIconForAccountType(accountType)}
            </Group>

            <Stack spacing="xs">
                <Text color="green" weight={500} fz="xl">
                    Balance: {getDollarText(balanceCents)}
                </Text>
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