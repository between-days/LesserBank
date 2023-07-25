import { getDollarTextFromCents, getIconForTransactionStatus, getPrettyDate } from "@/UIUtils";
import { Transaction, TransactionStatus } from "@/interfaces";
import { Group, Paper, Text } from "@mantine/core";
import { useHover } from "@mantine/hooks";
import { TextAndSubtext } from "../shared/TextAndSubtext";

export const getPropsFromTransaction = (transaction: Transaction, accountNumber: string): TransactionCardAltProps => {
    let otherNumber: string
    let otherBsb: string
    let otherName: string | undefined = ""
    let fromThis: boolean

    if (transaction.fromNumber === accountNumber) {
        otherNumber = transaction.toNumber
        otherBsb = transaction.toBsb
        otherName = transaction.toName
        fromThis = true
    } else {
        otherNumber = transaction.fromNumber
        otherBsb = transaction.fromBsb
        otherName = transaction.toName
        fromThis = false
    }

    return { ...{ ...transaction, otherNumber, otherBsb, otherName, fromThis } }
}

interface TransactionCardAltProps {
    amountCents: number
    availableBalanceCents: number
    transactionStatus: TransactionStatus
    dateStart: string
    dateEnd?: string
    fromThis: boolean
    otherNumber: string
    otherBsb: string
    otherName?: string
}

const TransactionItem = ({ amountCents, availableBalanceCents, transactionStatus, dateStart, dateEnd, fromThis, otherNumber, otherBsb, otherName }: TransactionCardAltProps) => {
    const { hovered, ref } = useHover();

    return (
        <Paper ref={ref} shadow={hovered ? "xl" : "sm"} radius="lg" pl="xl" pr="xl" p="xs">
            <Group position="apart">
                <TextAndSubtext text={getDollarTextFromCents(amountCents)} subText={getPrettyDate(dateEnd || dateStart)} />

                <Group>
                    <Text color="dimmed">{fromThis ? "To" : "From"}</Text>
                    <TextAndSubtext text={otherNumber} subText={otherBsb} textSize="sm" subTextSize="xs" />
                </Group>

                <Group>
                    {getIconForTransactionStatus(transactionStatus)}
                    <TextAndSubtext text={getDollarTextFromCents(availableBalanceCents)} subText="Available" textSize="md" subTextSize="sm" />
                </Group>
            </Group>
        </Paper>
    );
}
export default TransactionItem;


