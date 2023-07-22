import { TRANSACTION_ICON_TO_FROM_SIZE } from "@/UIConstants";
import { getAccountNumberString, getDollarTextFromCents, getIconForTransactionStatus, getPrettyDate } from "@/UIUtils";
import { TransactionStatus } from "@/interfaces";
import { Flex, Group, Paper, Stack, Text, ThemeIcon, useMantineTheme } from "@mantine/core";
import { useHover } from "@mantine/hooks";
import { IconArrowBadgeRight } from "@tabler/icons-react";

interface TransactionCardProps {
    fromNumber: number
    fromBsb: number
    toNumber: number
    toBsb: number
    fromName: string | undefined
    toName: string | undefined
    amountCents: number
    availableBalanceCents: number
    status: TransactionStatus
    date: string
}

export default function TransactionItem({ fromNumber, fromBsb, toNumber, toBsb, fromName, toName, amountCents, availableBalanceCents, status, date, }: TransactionCardProps) {
    const { hovered, ref } = useHover();
    const theme = useMantineTheme();

    return (
        <Paper ref={ref} shadow={hovered ? "xl" : "sm"} radius="lg" pl="xl" pr="xl" p="xs">
            <Group position="apart">
                <Flex mih={50} align="flex-start" direction="column" wrap="wrap" >
                    <Text fz="lg" fw={500}>
                        {getDollarTextFromCents(amountCents)}
                    </Text>
                    <Text fz="md" color="dimmed">
                        {getPrettyDate(date)}
                    </Text>

                </Flex>
                <Flex mih={50} gap="xl" justify="flex-start" align="center" direction="row" wrap="wrap">
                    <div >
                        <Text fz="sm" fw={500}>{fromName}</Text>
                        <Text fz="xs" color="dimmed">{getAccountNumberString(fromNumber)}</Text>
                    </div>

                    <ThemeIcon size={TRANSACTION_ICON_TO_FROM_SIZE} variant="light" radius="md">
                        <IconArrowBadgeRight />
                    </ThemeIcon>

                    <div>
                        <Text fz="sm" fw={500}>{toName}</Text>
                        <Text fz="xs" color="dimmed">{getAccountNumberString(toNumber)}</Text>
                    </div>
                </Flex>

                <Group>
                    {getIconForTransactionStatus(status)}
                    <Stack spacing={0}>
                        <Text fz="md" fw={500}>{getDollarTextFromCents(availableBalanceCents)}</Text>
                        <Text fz="sm" color="dimmed">Available</Text>
                    </Stack>
                </Group>
            </Group>
        </Paper>
    );
}
