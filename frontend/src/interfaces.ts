export type AccountStatus = "active" | "inactive"
export type TransactionStatus = "complete" | "pending" | "error"
export type AccountType = "savings" | "termDeposit" | "transaction"

export interface Account {
  dateOpened: string
  status: AccountStatus
  name: string | undefined
  accountType: AccountType
  balanceCents: number
  availableBalanceCents: number
  accountNumber: number
  bsb: number
}

export interface Accounts {
  accounts: Account[]
}

export interface InternalTransaction {
  fromNumber: number
  fromBsb: number
  toNumber: number
  toBsb: number
  fromName: string | undefined
  toName: string | undefined
  amountCents: number
  availableBalanceCents: number
  status: TransactionStatus
  date: string
}

export interface ExternalTransaction {
  fromNumber: number
  fromBsb: number
  toNumber: number
  toBsb: number
  fromName: string | undefined
  toName: string | undefined
  amountCents: number
  availableBalanceCents: number
  status: TransactionStatus
  date: string
  toBusinessName: string
}