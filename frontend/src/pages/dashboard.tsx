import { Container, SimpleGrid } from '@mantine/core';
import AccountCard from '@/components/AccountCard';
import CustomAppShell from '@/components/CustomAppShell';

interface Account {
  name: string
  accountType: string
  balanceCents: number
  accountNumber: number
  bsb: number
}

function mockAccounts(): Account[] {
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

function DashboardContent() {

  const accounts = mockAccounts().map((account, i) => <div>
    <AccountCard {...account} />
  </div>
  );

  return <Container size="xl">
    <SimpleGrid
      cols={3}
      spacing="xl"
      verticalSpacing="xl"

      breakpoints={[
        { maxWidth: 'md', cols: 3, spacing: 'md' },
        { maxWidth: 'sm', cols: 2, spacing: 'sm' },
        { maxWidth: 'xs', cols: 1, spacing: 'sm' },
      ]}

    >
      {accounts}
    </SimpleGrid>
  </Container>
}

export default function Dashboard() {
  return (
    <CustomAppShell>
      <DashboardContent />
    </CustomAppShell>
  )
}