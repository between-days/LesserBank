import AccountCard from "@/components/AccountCard";
import CustomAppShell from "@/components/CustomAppShell";
import TransactionCard from "@/components/TransactionCard";
import { Account } from "@/interfaces";
import { mockFindAccountByNumber, mockTransactions } from "@/mockBackend";
import { Card, Container, Space, Title } from "@mantine/core";

function AccountDetailContent(accountNumber: number) {
    const account: Account | undefined = mockFindAccountByNumber(accountNumber)

    if (!account) {
        return <>
            Not Found
        </>
    }

    const transactions = mockTransactions()

    const transactionsContent = transactions.map((transaction, i) =>
        <div key={i}>
            <TransactionCard {...transaction} />
            <Space h="md" />
        </div>)

    return <Container size="xl">
        <Card>
            <AccountCard {...{ ...account, onHover: false }} />
            <Space h="xl" />
            <Title>Transactions</Title>
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

    return (
        <CustomAppShell {...{ title: `Account ${accountNumber}` }}>
            {accountDetailContent}
        </CustomAppShell>
    )
}