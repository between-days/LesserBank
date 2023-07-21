import useSWR, { SWRResponse } from 'swr'
import { BASE_URL } from './Constants'
import { Accounts } from '@/interfaces'

const fetcher = (...args: any[]) => fetch(...args).then((res) => res.json())

export interface FindAccountQuery {
    accountId?: number
    accountNumber?: number
}

// TODO: prepend all api routes with /api
export const useAccounts = (query?: FindAccountQuery): SWRResponse<Accounts> => {
    let queryString = ""
    if (query?.accountId || query?.accountNumber) queryString += "?"
    queryString + new URLSearchParams({
        ...(query?.accountId && { accountId: query?.accountId.toString() }),
        ...(query?.accountNumber && { accountN: query?.accountNumber.toString() })
    }).toString()

    return useSWR(`${BASE_URL}/customers/1/accounts${queryString}`, fetcher)
}