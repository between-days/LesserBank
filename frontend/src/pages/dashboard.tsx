import { Affix, Button, Grid, SegmentedControl, Title, rem } from '@mantine/core';
import React from 'react';
import CustomPage from '@/components/shared/CustomPage';
import { IconSquareRoundedPlus } from '@tabler/icons-react';
import AccountCard from '@/components/account/AccountCard';
import { InternalErrorContent } from '@/components/shared/error/InternalErrorContent';
import { useAccounts } from '@/services/AccountsService';

const DashboardAffixes = () => {
    return <>
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
    </>
}

export default function Dashboard() {
    const { data, error } = useAccounts();

    if (error) return <InternalErrorContent />
    if (!data) return <>Loading...</>

    const accounts = data

    return (
        <CustomPage title={
            <Title order={2}>Accounts</Title>
        }>
            <DashboardAffixes />
            <Grid gutter="xl">
                {accounts.map((account, i) =>
                    <Grid.Col md={4} key={i}>
                        <AccountCard {...{ ...account, onHover: true }} />
                    </Grid.Col>
                )}
            </Grid>
        </CustomPage>
    )
}
