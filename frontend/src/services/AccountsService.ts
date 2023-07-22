import useSWR, { SWRResponse } from 'swr'
import { BASE_URL, CUSTOMER_ID } from './Constants'
import { Account, AccountType, Accounts } from '@/interfaces'
import apiFetcher from './ApiFetcher'

export interface FindAccountQuery {
    accountId?: number
    accountNumber?: number
    accountType?: AccountType
}

function buildFindAccountsQueryString(query: FindAccountQuery) {
    if (!(query.accountId || query.accountNumber)) return ""

    let queryString = "?"

    queryString += new URLSearchParams({
        ...(query.accountId && { accountId: query.accountId.toString() }),
        ...(query.accountNumber && { accountNumber: query.accountNumber.toString() })
    }).toString()

    return queryString
}

export const useAccounts = (query?: FindAccountQuery): SWRResponse<Account[]> => {
    let queryString = query ? buildFindAccountsQueryString(query) : ""

    const fetcher = (url: string) => apiFetcher<Accounts>(url).then(data => data.accounts)

    return useSWR(`${BASE_URL}/customers/${CUSTOMER_ID}/accounts${queryString}`, fetcher)
}

export const useOneAccount = (query: FindAccountQuery): SWRResponse<Account> => {
    let queryString = buildFindAccountsQueryString(query)

    const fetcher = (url: string) => apiFetcher<Accounts>(url).then(data => {
        if (data.accounts.length > 1) {
            throw new Error("Shouldn't be more than one", { cause: data })
        } else if (data.accounts.length == 0) {
            throw new Error("No account found", { cause: data })
        }
        return data.accounts[0]
    })

    return useSWR(`${BASE_URL}/customers/${CUSTOMER_ID}/accounts${queryString}`, fetcher)
}