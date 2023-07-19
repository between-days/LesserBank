export type AccountStatus = "active" | "inactive"
export type TransactionStatus = "complete" | "pending" | "error"

export interface Account {
  dateOpened: Date
  status: AccountStatus
  name: string
  accountType: string
  balanceCents: number
  availableBalanceCents: number
  accountNumber: number
  bsb: number
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
  date: Date
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
  status: "complete" | "pending" | "error"
  date: Date
  toBusinessName: string
}

export interface AccountCardProps {
  name: string
  accountType: string
  balanceCents: number
  accountNumber: number
  bsb: number,
  onHover: boolean
}