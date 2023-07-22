import { getDollarTextFromCents, getIconForTransactionStatus, getPrettyDate } from "@/UIUtils";
import { TransactionStatus } from "@/interfaces";
import { Group, Paper } from "@mantine/core";
import { useHover } from "@mantine/hooks";

import { TransactionToFromItem } from "./TransactionToFromItem";
import { TextAndSubtext } from "../shared/TextAndSubtext";

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

    return (
        <Paper ref={ref} shadow={hovered ? "xl" : "sm"} radius="lg" pl="xl" pr="xl" p="xs">
            <Group position="apart">
                <TextAndSubtext text={getDollarTextFromCents(amountCents)} subText={getPrettyDate(date)} />

                <TransactionToFromItem fromName={fromName} fromNumber={fromNumber} toName={toName} toNumber={toNumber} />

                <Group>
                    {getIconForTransactionStatus(status)}
                    <TextAndSubtext text={getDollarTextFromCents(availableBalanceCents)} subText="Available" textSize="md" subTextSize="sm" />
                </Group>
            </Group>
        </Paper>
    );
}
