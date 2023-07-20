import { TRANSACTION_ICON_SIZE } from "@/UIConstants";
import { getAccountNumberString, getDollarTextFromCents } from "@/UIUtils";
import { TransactionStatus } from "@/interfaces";
import { Card, Flex, Text, ThemeIcon, useMantineTheme } from "@mantine/core";
import { useHover } from "@mantine/hooks";
import { IconArrowBadgeRight, IconArrowBigRight, IconArrowBigRightLines, } from "@tabler/icons-react";

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
    date: Date
}

export default function TransactionCard({ fromNumber, fromBsb, toNumber, toBsb, fromName, toName, amountCents, availableBalanceCents, status, date, }: TransactionCardProps) {
    const { hovered, ref } = useHover();
    const theme = useMantineTheme();

    return (
        <Card ref={ref} shadow={hovered ? "xl" : "sm"} radius="md" padding="md">
            <Flex justify="space-between" wrap="wrap" align="center" >
                <Flex mih={50} align="flex-start" direction="column" wrap="wrap" >
                    <Text fz="md" color={theme.primaryColor}>
                        {getDollarTextFromCents(amountCents)}
                    </Text>
                    <Text fz="md" color="dimmed">
                        {date.toLocaleDateString('en-au')}
                    </Text>

                </Flex>
                <Flex mih={50} gap="md" justify="flex-start" align="center" direction="row" wrap="wrap">
                    <div >
                        <Text fz="sm" fw={500}>{fromName}</Text>
                        <Text fz="xs" color="dimmed">{getAccountNumberString(fromNumber)}</Text>
                    </div>

                    <ThemeIcon size={TRANSACTION_ICON_SIZE} variant="light" radius="md">
                        <IconArrowBadgeRight />
                    </ThemeIcon>

                    <div>
                        <Text fz="sm" fw={500}>{toName}</Text>
                        <Text fz="xs" color="dimmed">{getAccountNumberString(toNumber)}</Text>
                    </div>
                </Flex>
                <Text fz="md" fw={350}> Available: {getDollarTextFromCents(availableBalanceCents)}</Text>
            </Flex>
        </Card>
    );
}

// TODO: status pending light, complete filled, error red - might need a new color set for reds