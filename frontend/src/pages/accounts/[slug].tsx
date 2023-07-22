import TransactionItem from "@/components/TransactionItem";
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

    // skeleton
    // const transactionsContent = Array(8).fill(0).map((_, i) =>
    // <div key={i}>
    //     <Skeleton w="100%" h="5rem" radius="lg" />
    //     <Space h="md" />
    // </div>

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