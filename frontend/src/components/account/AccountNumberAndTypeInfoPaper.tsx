import { getAccountNumberString, getBsbString, getIconForAccountType } from "@/UIUtils"
import { AccountType } from "@/interfaces"
import { Group, Paper } from "@mantine/core"
import { TextAndSubtext } from "../shared/TextAndSubtext"

interface AccountNumberAndTypeInfoPaperProps {
    accountNumber: string,
    bsb: number,
    accountType: AccountType
}

export const AccountNumberAndTypeInfoPaper = ({ accountNumber, accountType, bsb }: AccountNumberAndTypeInfoPaperProps) => {
    return <Paper withBorder shadow="sm" radius="lg" p="md">
        <Group>
            {getIconForAccountType(accountType)}
            <TextAndSubtext text={getAccountNumberString(accountNumber)} subText={getBsbString(bsb)} textSize="md" subTextSize="xs" />
        </Group>
    </Paper>
}