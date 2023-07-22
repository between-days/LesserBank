import { InternalTransaction } from "./interfaces"

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
      date: new Date(Date.now()).toString()
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
      date: new Date(Date.now()).toString()
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
      date: new Date(Date.now()).toString()
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
      date: new Date(Date.now()).toString()
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
      date: new Date(Date.now()).toString()
    },
  ]
  return internalTransactions
}