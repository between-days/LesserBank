import { Card, Text, Group, Stack, Flex } from '@mantine/core';
import { useHover } from '@mantine/hooks';
import { useRouter } from 'next/router';
import { getAccountCardImage, getAccountNumberString, getBsbString, getDollarText, getIconForAccountType } from '@/UIUtils';
import { AccountCardProps } from '@/interfaces';

export default function AccountCard({ name, accountType, balanceCents, accountNumber, bsb }: AccountCardProps) {
    const { hovered, ref } = useHover();
    const router = useRouter()

    return (
        <Card ref={ref} onClick={() => { router.push(`/accounts/${accountNumber}`) }} shadow={hovered ? "xl" : "sm"} radius="md" withBorder={hovered ? false : true}>
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