import { Container, SimpleGrid } from '@mantine/core';
import AccountCard from '@/components/AccountCard';
import CustomAppShell from '@/components/CustomAppShell';
import { mockAccounts } from '@/mockBackend';

function DashboardContent() {
  const accounts = mockAccounts().map((account, i) => <div key={i}>
    <AccountCard {...{ ...account, onHover: true }} />
  </div>
  );

  return <Container size="xl" id="dashboard-container">
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
    <CustomAppShell {...{ title: "Accounts" }}>
      <DashboardContent />
    </CustomAppShell>
  )
}