import { getAccountNumberString, getBsbString, getIconForAccountType } from "@/UIUtils"
import { AccountType } from "@/interfaces"
import { Group, Paper, Stack, Text } from "@mantine/core"

interface AccountNumberAndTypeInfoPaperProps {
    accountNumber: number,
    bsb: number,
    accountType: AccountType
}

export const AccountNumberAndTypeInfoPaper = ({ accountNumber, accountType, bsb }: AccountNumberAndTypeInfoPaperProps) => {
    return <Paper withBorder shadow="sm" radius="lg" p="md" key={"aaaaa"}>
        <Group>
            {getIconForAccountType(accountType)}
            <Stack spacing={0}>
                <Text size="md" weight={500}>
                    {getAccountNumberString(accountNumber)}
                </Text>
                <Text size="xs" color="dimmed">
                    {getBsbString(bsb)}
                </Text>
            </Stack>
        </Group>
    </Paper>
}