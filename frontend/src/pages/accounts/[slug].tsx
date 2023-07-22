import TransactionItem from "@/components/transaction/TransactionItem";
import { mockTransactions } from "@/mockBackend";
import { Grid, Group, Skeleton, Space, Stack, Title, useMantineTheme, } from "@mantine/core";
import React from 'react';
import CustomPage from "@/components/shared/CustomPage";
import { AccountNumberAndTypeInfoPaper } from "@/components/account/AccountNumberAndTypeInfoPaper";
import { AccountOpenedInfoPaper } from "@/components/account/AccountOpenedInfoPaper";
import { AccountBalancesInfoPaper } from "@/components/account/AccountBalancesInfoPaper";
import { useOneAccount } from "@/services/AccountsService";
import { InternalErrorContent } from "@/components/shared/error/InternalErrorContent";

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

    const accountNumber = Number(slug)

    const { data, error } = useOneAccount({ accountNumber })
    const transactions = mockTransactions()

    if (error) return <InternalErrorContent />

    const transactionsContent = transactions.map((transaction, i) =>
        <div key={i}>
            <TransactionItem {...transaction} />
            <Space h="md" />
        </div>)

    if (!accountNumber || !data) return <AccountDetailsLoadingContent>
        {transactionsContent}
    </AccountDetailsLoadingContent>

    const account = data

    const { name, dateOpened, balanceCents, availableBalanceCents, accountType, bsb } = account

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
                        {transactionsContent}
                    </div>
                </Grid.Col>
            </Grid>
        </CustomPage>
    )
}