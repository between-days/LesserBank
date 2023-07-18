export interface Account {
  name: string
  accountType: string
  balanceCents: number
  accountNumber: number
  bsb: number
}

interface Transaction {
  fromNumber: number
  toNumber: number
  fromName: string
  toName: string
  amountCents: number
}


export interface AccountCardProps {
  name: string
  accountType: string
  balanceCents: number
  accountNumber: number
  bsb: number
}