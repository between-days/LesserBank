import { Title } from '@mantine/core';
import React from 'react';
import CustomPage from '@/components/shared/CustomPage';
import { DashboardContent } from '../components/account/content/DashboardContent';
import { DashboardAffixes } from '../components/account/content/DashboardAffixes';

export default function Dashboard() {
    return (
        <CustomPage title={
            <Title order={2}>Accounts</Title>
        }>
            <DashboardAffixes />
            <DashboardContent />
        </CustomPage>
    )
}
