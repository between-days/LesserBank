import TransactionItem from "@/components/transaction/TransactionItem";
import { Grid, Group, Skeleton, Space, Stack, Title, useMantineTheme, } from "@mantine/core";
import React from 'react';
import CustomPage from "@/components/shared/CustomPage";
import { AccountNumberAndTypeInfoPaper } from "@/components/account/AccountNumberAndTypeInfoPaper";
import { AccountOpenedInfoPaper } from "@/components/account/AccountOpenedInfoPaper";
import { AccountBalancesInfoCard } from "@/components/account/AccountBalancesInfoCard";
import { useOneAccount } from "@/services/AccountsService";
import { InternalErrorContent } from "@/components/shared/error/InternalErrorContent";
import { useTransactions } from "@/services/TransactionsService";
import TransactionItemAlt, { getPropsFromTransaction } from "@/components/transaction/TransactionItemAlt";

export const getServerSideProps = async (context: { query: { slug: any; }; }) => {
    let { slug } = context.query;
    return { props: { slug: slug } };
};

const AccountDetailsLoadingContent = ({ children }: { children: any }) => {
    return (
        <CustomPage title={
            <Title color="dimmed" >
                Loading Account Details...
            </Title>
        }>
            <Grid gutter="md">
                <Grid.Col xs={4}>
                    <Skeleton h="11rem" pl="5rem" radius="lg" />
                </Grid.Col>
                <Grid.Col xs={4}>
                    <Skeleton h="11rem" pl="5rem" radius="lg" />
                </Grid.Col>
                <Grid.Col xs={4}>
                    <Stack align="stretch" justify="center">
                        <Skeleton w="20rem" h="5rem" radius="lg" />
                        <Skeleton w="20rem" h="5rem" radius="lg" />
                    </Stack>
                </Grid.Col>

                <Grid.Col xs={12} >
                    <div>
                        <Title order={2} color="dimmed">Transactions</Title>
                        <Space mb="md" />
                        {children}
                    </div>
                </Grid.Col>
            </Grid>
        </CustomPage>
    )
}

export default function AccountDetail(props: { slug: any; }) {
    const { slug } = props

    const theme = useMantineTheme()

    const accountNumber = slug

    const { data: account, error: accountsError } = useOneAccount({ accountNumber })
    const { data: transactions, error: transactionsError } = useTransactions({ accountNumber })

    if (accountsError) return <InternalErrorContent />
    if (transactionsError) return <InternalErrorContent />

    let transactionsContent: any
    if (accountNumber && transactions) {
        transactionsContent = <div>
            {transactions.length > 0 && <>
                <Title order={2}>Transactions</Title>
                <Space mb="md" />
            </>}

            {transactions.map((transaction, i) => {
                let trItemProps = getPropsFromTransaction(transaction, accountNumber)

                return <div key={i}>
                    <TransactionItemAlt {...trItemProps} />
                    <Space h="md" />
                </div>
            })
            }
        </div>
    } else {
        transactionsContent = <>Loading...</>
    }

    if (!accountNumber || !account) return <AccountDetailsLoadingContent>
        {transactionsContent}
    </AccountDetailsLoadingContent>

    const { name, dateOpened, balanceCents, availableBalanceCents, accountType, bsb } = account

    return (
        <CustomPage title={
            <Group align="flex-start">
                <Title order={2}>Account Details for</Title>
                <Title order={2} color={theme.primaryColor} italic>
                    {name ? name : accountNumber}
                </Title>
            </Group>
        }>
            <Grid align="center">
                <Grid.Col xs={12} sm={4}>
                    <AccountBalancesInfoCard availableBalanceCents={availableBalanceCents} balanceCents={balanceCents} />
                </Grid.Col>
                <Grid.Col xs={12} sm={4}>
                    <Stack align="stretch" justify="center">
                        <AccountNumberAndTypeInfoPaper accountNumber={accountNumber} accountType={accountType} bsb={bsb} />
                        <AccountOpenedInfoPaper dateOpened={dateOpened} />
                    </Stack>
                </Grid.Col>
                {/* <Grid.Col xs={3}>
                    <Card withBorder shadow="sm" radius="lg" p="md" h="11rem">
                        <Center>
                            <Button leftIcon={<IconArrowBarToRight size="1rem" style={{marginRight: "1rem"}}/>} variant="filled" > New Transaction</Button>
                        </Center>
                    </Card>
                </Grid.Col> */}
                <Grid.Col xs={12} sm={8}>
                    {transactionsContent}
                </Grid.Col>
            </Grid>

        </CustomPage>
    )
}