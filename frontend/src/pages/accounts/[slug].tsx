import AccountCard from "@/components/AccountCard";
import CustomAppShell from "@/components/CustomAppShell";
import NotFoundContent from "@/components/NotFoundContent";
import TransactionCard from "@/components/TransactionCard";
import { Account } from "@/interfaces";
import { mockTransactions } from "@/mockBackend";
import { Card, Container, Space, Title } from "@mantine/core";
import React, { useState, useEffect } from 'react';

function AccountDetailContent(accountNumber: number) {
    const [account, setAccount] = useState<Account | null>(null)
    const [isLoading, setLoading] = useState(false)

    // TODO: /api prepend for all api routes
    useEffect(() => {
        setLoading(true)
        fetch(`http://localhost:8080/customers/1/accounts?accountNumber=${accountNumber}`)
            .then((res) => res.json())
            .then((data) => {
                console.log("DATA GOT")
                console.log(data)
                const account = data.accounts[0];
                setAccount(account)
                setLoading(false)
            })
    }, [])

    if (isLoading) return <p>Loading...</p>

    if (!account) return <NotFoundContent />

    const transactions = mockTransactions()

    const transactionsContent = transactions.map((transaction, i) =>
        <div key={i}>
            <TransactionCard {...transaction} />
            <Space h="md" />
        </div>)

    return <Container size="sm">
        <Card>
            <AccountCard {...{ ...account, onHover: false }} />
            <Space h="xl" />
            <Title order={2}>Transactions</Title>
            <Space h="xl" />
            {transactionsContent}
        </Card>
    </Container>
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
        <CustomAppShell {...{ title }}>
            {accountDetailContent}
        </CustomAppShell>
    )
}