import { getAccountNumberString, getBsbString, getDollarTextFromCents, getIconForTransactionStatus, getPrettyDate } from "@/UIUtils";
import { TransactionStatus, TransactionType } from "@/interfaces";
import { Group, Paper, ThemeIcon } from "@mantine/core";
import { useHover } from "@mantine/hooks";
import { TextAndSubtext } from "../shared/TextAndSubtext";
import { TRANSACTION_ICON_TO_FROM_SIZE } from "@/UIConstants";
import { IconArrowBadgeRight } from "@tabler/icons-react";

interface TransactionToFromItemProps {
    fromName?: string
    fromNumber: string
    toName?: string
    toNumber: string
    fromBsb?: string
    toBsb?: string
}

export const TransactionToFromItem = ({ fromName, toName, fromNumber, toNumber, fromBsb, toBsb }: TransactionToFromItemProps) => {
    return <Group mih={50} spacing="lg" align="center">

        {fromName ? <TextAndSubtext text={fromName} subText={getAccountNumberString(fromNumber)} textSize="sm" subTextSize="xs" /> : <TextAndSubtext text={getAccountNumberString(fromNumber)} subText={getBsbString(fromBsb || "")} textSize="sm" subTextSize="xs" />}

        <ThemeIcon size={TRANSACTION_ICON_TO_FROM_SIZE} variant="light" radius="md">
            <IconArrowBadgeRight />
        </ThemeIcon>

        {toName ? <TextAndSubtext text={toName} subText={getAccountNumberString(toNumber)} textSize="sm" subTextSize="xs" /> : <TextAndSubtext text={getAccountNumberString(toNumber)} subText={getBsbString(toBsb || "")} textSize="sm" subTextSize="xs" />}
    </Group>
}

interface TransactionCardProps {
    transactionType: TransactionType,
    fromNumber: string
    fromBsb: string
    toNumber: string
    toBsb: string
    fromName: string | undefined
    toName: string | undefined
    amountCents: number
    availableBalanceCents: number
    transactionStatus: TransactionStatus
    dateStart: string
    dateEnd?: string
}

const TransactionItem = ({ fromNumber, toNumber, fromName, toName, amountCents, availableBalanceCents, transactionStatus, dateStart, dateEnd, fromBsb, toBsb }: TransactionCardProps) => {
    const { hovered, ref } = useHover();

    return (
        <Paper ref={ref} shadow={hovered ? "xl" : "sm"} radius="lg" pl="xl" pr="xl" p="xs">
            <Group position="apart">
                <TextAndSubtext text={getDollarTextFromCents(amountCents)} subText={getPrettyDate(dateEnd || dateStart)} />

                <TransactionToFromItem fromName={fromName} fromNumber={fromNumber} toName={toName} toNumber={toNumber} fromBsb={fromBsb} toBsb={toBsb} />

                <Group>
                    {getIconForTransactionStatus(transactionStatus)}
                    <TextAndSubtext text={getDollarTextFromCents(availableBalanceCents)} subText="Available" textSize="md" subTextSize="sm" />
                </Group>
            </Group>
        </Paper>
    );
}
export default TransactionItem;


