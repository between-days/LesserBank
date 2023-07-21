import AccountCard from "@/components/AccountCard";
import CustomPage from "@/components/shared/CustomPage";
import { InternalErrorContent } from "@/components/shared/error/InternalErrorContent";
import TransactionCard from "@/components/TransactionCard";
import { mockTransactions } from "@/mockBackend";
import { useAccounts } from "@/services/AccountsService";
import { getAccountNumberString, getBsbString, getDollarTextFromCents, getIconForAccountType, getPrettyDate, getPrettyDateTime } from "@/UIUtils";
import { Grid, Group, Paper, Space, Stack, Text, ThemeIcon, Title, Tooltip, useMantineTheme } from "@mantine/core";
import { IconCalendar } from "@tabler/icons-react";
import React from 'react';

function AccountDetailContent(accountNumber: number) {
    const theme = useMantineTheme();
    const { data, error } = useAccounts({ accountNumber })

    if (error) return <InternalErrorContent />
    if (!data) return <>Loading</>

    let account = data.accounts[0]

    const transactions = mockTransactions()

    const transactionsContent = transactions.map((transaction, i) =>
        <div key={i}>
            <TransactionCard {...transaction} />
            <Space h="md" />
        </div>)

    return <Grid>
        <Grid.Col xs={12} mb="md">
            <Group>
                <Title order={2}>Account Details for</Title>
                <Title order={2} color={theme.primaryColor} italic>
                    {account.name ? account.name : accountNumber}
                </Title>
            </Group>

        </Grid.Col>
        <Grid.Col xs={4}>
            <AccountCard {...{ ...data.accounts[0], onHover: false }} />
        </Grid.Col>
        <Grid.Col xs={4}>
            <Paper withBorder shadow="sm" radius="lg" p="md" key={"aaaaa"} h="11rem">
                <Stack>
                    <Stack spacing={0}>
                        <Text size="lg" weight={500}>
                            {getDollarTextFromCents(account.availableBalanceCents)}
                        </Text>
                        <Text size="md" color="dimmed">
                            Available Balance
                        </Text>
                    </Stack>
                    <Stack spacing={0}>
                        <Text size="lg" weight={500}>
                            {getDollarTextFromCents(account.balanceCents)}
                        </Text>
                        <Text size="md" color="dimmed">
                            Balance
                        </Text>
                    </Stack>
                </Stack>
            </Paper>
        </Grid.Col>
        <Grid.Col xs={4}>
            <Stack align="stretch" justify="center">
                <Paper withBorder shadow="sm" radius="lg" p="md" key={"aaaaa"}>
                    <Group>
                        {getIconForAccountType(account.accountType)}
                        <Stack spacing={0}>
                            <Text size="md" weight={500}>
                                {getAccountNumberString(account.accountNumber)}
                            </Text>
                            <Text size="xs" color="dimmed">
                                {getBsbString(account.bsb)}
                            </Text>
                        </Stack>
                    </Group>
                </Paper>
                <Paper withBorder shadow="sm" radius="lg" p="md" key={"aaaaa"} h="5rem">
                    <Group>
                        <Tooltip label={getPrettyDateTime(account.dateOpened)}>
                            <ThemeIcon size="xl" variant="light" radius="md">
                                <IconCalendar />
                            </ThemeIcon>
                        </Tooltip>
                        <Stack spacing={0}>
                            <Text fz="md" fw={500}>
                                {getPrettyDate(account.dateOpened)}
                            </Text>
                            <Text color="dimmed" fz="xs">
                                Opened
                            </Text>
                        </Stack>
                    </Group>
                </Paper>
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









}

export const getServerSideProps = async (context: { query: { slug: any; }; }) => {
    let { slug } = context.query;
    return { props: { slug: slug } };
};

export default function AccountDetail(props: { slug: any; }) {
    const { slug } = props

    if (!slug) return <></>

    const accountNumber = Number(slug)

    const accountDetailContent = AccountDetailContent(accountNumber)

    const title = accountNumber ? `Account ${accountNumber}` : "Error"

    return (
        <CustomPage title={title}>
            {accountDetailContent}
        </CustomPage>
    )
}