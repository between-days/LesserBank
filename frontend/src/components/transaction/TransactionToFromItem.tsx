import { TRANSACTION_ICON_TO_FROM_SIZE } from "@/UIConstants"
import { getAccountNumberString } from "@/UIUtils"
import { Group, Stack, Text, ThemeIcon } from "@mantine/core"
import { IconArrowBadgeRight } from "@tabler/icons-react"
import { TextAndSubtext } from "../shared/TextAndSubtext"

interface TransactionToFromItemProps {
    fromName?: string,
    fromNumber: number,
    toName?: string,
    toNumber: number
}

export const TransactionToFromItem = ({ fromName, toName, fromNumber, toNumber }: TransactionToFromItemProps) => {
    return <Group mih={50} spacing="lg" align="center">
        <TextAndSubtext text={fromName || ""} subText={getAccountNumberString(fromNumber)} textSize="sm" subTextSize="xs" />

        <ThemeIcon size={TRANSACTION_ICON_TO_FROM_SIZE} variant="light" radius="md">
            <IconArrowBadgeRight />
        </ThemeIcon>

        <TextAndSubtext text={toName || ""} subText={getAccountNumberString(toNumber)} textSize="sm" subTextSize="xs" />
    </Group>
}