import CustomPage from "@/components/shared/CustomPage"
import { InternalErrorContent } from "@/components/shared/error/InternalErrorContent"
import { useOneAccount } from "@/services/AccountsService"
import { Grid, Group, Space, Stack, Title, useMantineTheme } from "@mantine/core"
import { AccountDetailLoadingContent } from "./AccountDetailsLoadingContent"
import { AccountOpenedInfoPaper } from "@/components/account/AccountOpenedInfoPaper"
import { AccountNumberAndTypeInfoPaper } from "@/components/account/AccountNumberAndTypeInfoPaper"
import { AccountBalancesInfoPaper } from "@/components/account/AccountBalancesInfoPaper"

interface AccountDetailContentProps {
    children: any
    accountNumber: number
}

export function AccountDetailContent({ children, accountNumber }: AccountDetailContentProps) {
    const theme = useMantineTheme()
    const { data, error } = useOneAccount({ accountNumber })

    if (error) {
        console.log("error: ", error)
        return <InternalErrorContent />
    }

    if (!data) return <AccountDetailLoadingContent accountNumber={accountNumber}>
        {children}
    </AccountDetailLoadingContent>

    const { name, dateOpened, balanceCents, availableBalanceCents, accountType, bsb } = data

    return (
        <CustomPage title={
            <Group>
                <Title order={2}>Account Details for</Title>
                <Title order={2} color={theme.primaryColor} italic>
                    {name ? name : accountNumber}
                </Title>
            </Group>
        }>
            <Grid>
                <Grid.Col xs={3}>
                    <AccountBalancesInfoPaper availableBalanceCents={availableBalanceCents} balanceCents={balanceCents} />
                </Grid.Col>
                <Grid.Col xs={4}>
                    <Stack align="stretch" justify="center">
                        <AccountNumberAndTypeInfoPaper accountNumber={accountNumber} accountType={accountType} bsb={bsb} />
                        <AccountOpenedInfoPaper dateOpened={dateOpened} />
                    </Stack>
                </Grid.Col>
                <Grid.Col xs={12} >
                    <div>
                        <Title order={2}>Transactions</Title>
                        <Space mb="md" />
                        {children}
                    </div>
                </Grid.Col>
            </Grid>
        </CustomPage>
    )
}