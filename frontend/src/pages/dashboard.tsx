import { Affix, Button, Container, Grid, SegmentedControl, Space, rem } from '@mantine/core';
import React from 'react';
import { IconSquareRoundedPlus } from '@tabler/icons-react';
import CustomPage from '@/components/shared/CustomPage';
import AccountCardAlt from '@/components/AccountCardAlt';
import { useAccounts } from '@/services/AccountsService';
import { InternalErrorContent } from '@/components/shared/error/InternalErrorContent';

function DashboardContent() {
  const { data, error } = useAccounts();

  if (error) return <InternalErrorContent />
  if (!data) return <>Loading</>

  return (<div>
    <Grid
      gutter="xl"
    >
      {data.accounts.map((account, i) =>
        <Grid.Col md={4} >
          <div key={i}>
            <AccountCardAlt {...{ ...account, onHover: true }} />
          </div>
        </Grid.Col>
      )}
    </Grid>
  </div>

  )
}

export default function Dashboard() {
  return (
    <CustomPage title="Accounts">
      <Affix position={{ bottom: rem(20), left: rem(20) }}>
        <Button
          radius="md"
          size="md"
          variant="gradient"
          gradient={{ from: 'orange', to: 'red' }}
          leftIcon={<IconSquareRoundedPlus size="1.5rem" />}
          onClick={() => { }}
        >
          New Account
        </Button>
      </Affix>
      <Affix position={{ top: rem(50), left: rem(20) }}>
        <SegmentedControl
          data={[
            { label: 'Grid', value: 'grid' },
            { label: 'Type', value: 'type', disabled: true },
          ]}
        />
      </Affix>
      <DashboardContent />
    </CustomPage>
  )
}
