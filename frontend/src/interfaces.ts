export type AccountStatus = "active" | "inactive"
export type AccountType = "savings" | "termDeposit" | "transaction"
export type TransactionStatus = "success" | "pending" | "error"
export type TransactionType = "internal" | "external"

export interface Account {
  dateOpened: string
  status: AccountStatus
  name: string | undefined
  accountType: AccountType
  balanceCents: number
  availableBalanceCents: number
  accountNumber: string
  bsb: string
}

export interface Accounts {
  accounts: Account[]
}

export interface Transaction {
  transactionType: TransactionType,
  fromNumber: string
  fromBsb: string
  toNumber: string
  toBsb: string
  fromName: string | undefined
  toName: string | undefined
  amountCents: number
  availableBalanceCents: number
  transactionStatus: TransactionStatus
  dateStart: string
  dateEnd: string
}

export interface Transactions {
  transactions: Transaction[]
}