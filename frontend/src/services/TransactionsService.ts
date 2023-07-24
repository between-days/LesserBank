import { Transaction, TransactionType, Transactions } from "@/interfaces"
import useSWR, { SWRResponse } from "swr"
import apiFetcher from "./ApiFetcher"
import { BASE_URL, CUSTOMER_ID } from "./Constants"

export interface FindTransactionQuery {
    transactionId?: number
    accountNumber?: number
    transactionType?: TransactionType
}

function buildFindTransactionsQueryString(query: FindTransactionQuery) {
    if (!(query.transactionId || query.accountNumber)) return ""

    let queryString = "?"

    queryString += new URLSearchParams({
        ...(query.transactionId && { transactionId: query.transactionId.toString() }),
        ...(query.accountNumber && { accountNumber: query.accountNumber.toString() })
    }).toString()

    return queryString
}

export const useTransactions = (query?: FindTransactionQuery): SWRResponse<Transaction[]> => {
    let queryString = query ? buildFindTransactionsQueryString(query) : ""

    const fetcher = (url: string) => apiFetcher<Transactions>(url).then(data => data.transactions)

    return useSWR(`${BASE_URL}/customers/${CUSTOMER_ID}/transactions${queryString}`, fetcher)
}