import { Account, InternalTransaction } from "./interfaces"

export function mockAccounts(): Account[] {
  const accounts: Account[] = [
    {
      dateOpened: new Date(Date.now()),
      status: "active",
      name: "Uhhhhggggg",
      accountType: "savings",
      balanceCents: 8766563,
      availableBalanceCents: 8766563,
      accountNumber: 625512586,
      bsb: 123456
    },
    {
      dateOpened: new Date(Date.now()),
      status: "active",
      name: "Sir Reginald the third",
      accountType: "savings",
      balanceCents: 345534543,
      availableBalanceCents: 8766563,
      accountNumber: 422079299,
      bsb: 123456
    },
    {
      dateOpened: new Date(Date.now()),
      status: "active",
      name: "Alphabet soup",
      accountType: "termDeposit",
      balanceCents: 33453443,
      availableBalanceCents: 8766563,
      accountNumber: 899731811,
      bsb: 123456
    },
    {
      dateOpened: new Date(Date.now()),
      status: "active",
      name: "I want a big shiny car",
      accountType: "savings",
      balanceCents: 3765843,
      availableBalanceCents: 8766563,
      accountNumber: 761458726,
      bsb: 123456
    },
    {
      dateOpened: new Date(Date.now()),
      status: "active",
      name: "Goats cheese is expensive",
      accountType: "termDeposit",
      balanceCents: 23498423443,
      availableBalanceCents: 8766563,
      accountNumber: 139660425,
      bsb: 123456
    },
    {
      dateOpened: new Date(Date.now()),
      status: "active",
      name: "Trampoline",
      accountType: "savings",
      balanceCents: 9874352343,
      availableBalanceCents: 8766563,
      accountNumber: 291048742,
      bsb: 123456
    },
    {
      dateOpened: new Date(Date.now()),
      status: "active",
      name: "Fork to eat soup with",
      accountType: "savings",
      balanceCents: 343,
      availableBalanceCents: 8766563,
      accountNumber: 454051231,
      bsb: 123456
    },
  ]

  return accounts
}

export function mockFindAccountByNumber(accountNumber: number): Account | undefined {
  const accounts: Account[] = mockAccounts()
  const account = accounts.find((account) => account.accountNumber == accountNumber)
  return account
}

export function mockTransactions() {
  const internalTransactions: InternalTransaction[] = [
    {
      fromNumber: 123456789,
      fromBsb: 123456,
      toNumber: 987654321,
      toBsb: 123456,
      fromName: "big money",
      toName: "big chungus",
      amountCents: 2634,
      availableBalanceCents: 43434,
      status: "complete",
      date: new Date(Date.now())
    },
    {
      fromNumber: 123456789,
      fromBsb: 123456,
      toNumber: 987654321,
      toBsb: 123456,
      fromName: "big money",
      toName: "big chungus",
      amountCents: 2634,
      availableBalanceCents: 43434,
      status: "complete",
      date: new Date(Date.now())
    }, {
      fromNumber: 123456789,
      fromBsb: 123456,
      toNumber: 987654321,
      toBsb: 123456,
      fromName: "big money",
      toName: "big chungus",
      amountCents: 234,
      availableBalanceCents: 43434,
      status: "complete",
      date: new Date(Date.now())
    }, {
      fromNumber: 123456789,
      fromBsb: 123456,
      toNumber: 987654321,
      toBsb: 123456,
      fromName: "big money",
      toName: "big chungus",
      amountCents: 2334,
      availableBalanceCents: 43434,
      status: "complete",
      date: new Date(Date.now())
    }, {
      fromNumber: 123456789,
      fromBsb: 123456,
      toNumber: 987654321,
      toBsb: 123456,
      fromName: "big money",
      toName: "big chungus",
      amountCents: 23634,
      availableBalanceCents: 43434,
      status: "complete",
      date: new Date(Date.now())
    },
  ]
  return internalTransactions
}