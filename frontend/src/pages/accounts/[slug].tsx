import TransactionItem from "@/components/transaction/TransactionItem";
import { mockTransactions } from "@/mockBackend";
import { Space, } from "@mantine/core";
import React from 'react';
import { AccountDetailContent } from "../../components/account/content/AccountDetailsContent";

export const getServerSideProps = async (context: { query: { slug: any; }; }) => {
    let { slug } = context.query;
    return { props: { slug: slug } };
};

export default function AccountDetail(props: { slug: any; }) {
    const { slug } = props

    if (!slug) return <></>

    const accountNumber = Number(slug)

    const transactions = mockTransactions()
    const transactionsContent = transactions.map((transaction, i) =>
        <div key={i}>
            <TransactionItem {...transaction} />
            <Space h="md" />
        </div>)

    return (
        <AccountDetailContent {...{ accountNumber }}>
            {transactionsContent}
        </AccountDetailContent>
    )
}