import { getFormattedAccountNumber, getFormattedBsb, getIconForAccountType } from "@/UIUtils"
import { AccountType } from "@/interfaces"
import { Group, Paper } from "@mantine/core"
import { TextAndSubtext } from "../shared/TextAndSubtext"

interface AccountNumberAndTypeInfoPaperProps {
    accountNumber: string,
    bsb: string,
    accountType: AccountType
}

export const AccountNumberAndTypeInfoPaper = ({ accountNumber, accountType, bsb }: AccountNumberAndTypeInfoPaperProps) => {
    return <Paper withBorder shadow="sm" radius="lg" p="md">
        <Group>
            {getIconForAccountType(accountType)}
            <TextAndSubtext text={getFormattedAccountNumber(accountNumber)} subText={getFormattedBsb(bsb)} textSize="md" subTextSize="xs" />
        </Group>
    </Paper>
}