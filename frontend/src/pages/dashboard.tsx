import { Container, SimpleGrid } from '@mantine/core';
import AccountCard from '@/components/AccountCard';
import CustomAppShell from '@/components/CustomAppShell';
import React, { useState, useEffect } from 'react';
import { Account } from '@/interfaces';

function DashboardContent() {
  const [accounts, setAccounts] = useState<Account[] | null>(null)
  const [isLoading, setLoading] = useState(false)

  // TODO: /api prepend for all api routes
  useEffect(() => {
    setLoading(true)
    fetch('http://localhost:8080/customers/1/accounts')
      .then((res) => res.json())
      .then((data) => {
        console.log("DATA GOT")
        console.log(data)
        setAccounts(data.accounts)
        setLoading(false)
      })
  }, [])

  if (isLoading) return <p>Loading...</p>
  if (!accounts) return <p>error getting accounts</p>

  const accountItems = accounts.map((account, i) => <div key={i}>
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
      {accountItems}
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
