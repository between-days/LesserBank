import { Affix, Button, Container, Grid, SegmentedControl, SimpleGrid, Space, Tooltip, Transition, rem } from '@mantine/core';
import AccountCard from '@/components/AccountCard';
import CustomAppShell from '@/components/CustomAppShell';
import React, { useState, useEffect } from 'react';
import { Account } from '@/interfaces';
import { IconArrowUp, IconSquarePlus, IconSquareRounded, IconSquareRoundedPlus } from '@tabler/icons-react';

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

  return <Container size="sm" id="dashboard-container">
    <Affix position={{ bottom: rem(20), right: rem(20) }}>
      <Button
        size="lg"
        variant="gradient"
        gradient={{ from: 'orange', to: 'red' }}
        leftIcon={<IconSquareRoundedPlus size="1.5rem" />}
        onClick={() => { }}
      >
        New Account
      </Button>
    </Affix>
    <Affix position={{ top: rem(100), right: rem(20) }}>
      <SegmentedControl
        data={[
          { label: 'Grid', value: 'grid' },
          { label: 'Type', value: 'type', disabled: true },
        ]}
      />
    </Affix>
    <Space h="xl" />
    <Grid
      gutter="xl"
    >
      {accounts.map((account, i) =>
        <Grid.Col md={6} >
          <div key={i}>
            <AccountCard {...{ ...account, onHover: true }} />
          </div>
        </Grid.Col>
      )}
    </Grid>
  </Container>
}

export default function Dashboard() {
  return (
    <CustomAppShell {...{ title: "Accounts" }}>
      <DashboardContent />
    </CustomAppShell>
  )
}
