
import { Account } from "./interfaces"

export function mockAccounts(): Account[] {
  const accounts = [
    {
      name: "Uhhhhggggg",
      accountType: "Savings",
      balanceCents: 8766563,
      accountNumber: 625512586,
      bsb: 123456
    },
    {
      name: "Sir Reginald the third",
      accountType: "Savings",
      balanceCents: 345534543,
      accountNumber: 422079299,
      bsb: 123456
    },
    {
      name: "Alphabet soup",
      accountType: "Term Deposit",
      balanceCents: 33453443,
      accountNumber: 899731811,
      bsb: 123456
    },
    {
      name: "I want a big shiny car",
      accountType: "Savings",
      balanceCents: 3765843,
      accountNumber: 761458726,
      bsb: 123456
    },
    {
      name: "Goats cheese is expensive",
      accountType: "Term Deposit",
      balanceCents: 23498423443,
      accountNumber: 139660425,
      bsb: 123456
    },
    {
      name: "Trampoline",
      accountType: "Savings",
      balanceCents: 9874352343,
      accountNumber: 291048742,
      bsb: 123456
    },
    {
      name: "Fork to eat soup with",
      accountType: "Savings",
      balanceCents: 343,
      accountNumber: 454051231,
      bsb: 123456
    },
  ]

  return accounts
}

export function mockFindAccountByNumber(accountNumber: number): Account | undefined {
  console.log("looking for account ", accountNumber)
  const accounts = mockAccounts()

  const account = accounts.find((account) => account.accountNumber == accountNumber)
  return account
}

export function mockTransactions() {
  const transactions = [
    {
      fromNumber: 123456789,
      toNumber: 987654321,
      fromName: "big money",
      toName: "big chungus",
      amountCents: 2345634
    },
    {
      fromNumber: 123456789,
      toNumber: 987654321,
      fromName: "big money",
      toName: "big chungus",
      amountCents: 2345634
    },
    {
      fromNumber: 123456789,
      toNumber: 987654321,
      fromName: "big money",
      toName: "big chungus",
      amountCents: 2345634
    },
    {
      fromNumber: 123456789,
      toNumber: 987654321,
      fromName: "big money",
      toName: "big chungus",
      amountCents: 2345634
    },
    {
      fromNumber: 123456789,
      toNumber: 987654321,
      fromName: "big money",
      toName: "big chungus",
      amountCents: 2345634
    },
    {
      fromNumber: 123456789,
      toNumber: 987654321,
      fromName: "big money",
      toName: "big chungus",
      amountCents: 2345634
    },
    {
      fromNumber: 123456789,
      toNumber: 987654321,
      fromName: "big money",
      toName: "big chungus",
      amountCents: 2345634
    },
  ]
  return transactions
}