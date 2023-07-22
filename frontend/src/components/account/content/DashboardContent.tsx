import { Grid } from "@mantine/core";
import AccountCard from "@/components/account/AccountCard";
import { InternalErrorContent } from "@/components/shared/error/InternalErrorContent";
import { useAccounts } from "@/services/AccountsService";

export const DashboardContent = () => {
    const { data, error } = useAccounts();

    if (error) return <InternalErrorContent />
    if (!data) return <>Loading...</>

    const accounts = data

    return (
        <Grid gutter="xl">
            {accounts.map((account, i) =>
                <Grid.Col md={4} key={i}>
                    <AccountCard {...{ ...account, onHover: true }} />
                </Grid.Col>
            )}
        </Grid>
    )
}