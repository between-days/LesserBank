import CustomPage from "@/components/shared/CustomPage"
import { Grid, Skeleton, Space, Stack, Title } from "@mantine/core"

export const AccountDetailLoadingContent = ({ children, accountNumber }: { children: any, accountNumber: number }) => {
    return (
        <CustomPage title={
            <Title color="dimmed" >
                {`Loading Account Details for ${accountNumber} ...`}
            </Title>
        }>
            <Grid gutter="md">
                <Grid.Col xs={4}>
                    <Skeleton h="11rem" pl="5rem" radius="lg" />
                </Grid.Col>
                <Grid.Col xs={4}>
                    <Skeleton h="11rem" pl="5rem" radius="lg" />
                </Grid.Col>
                <Grid.Col xs={4}>
                    <Stack align="stretch" justify="center">
                        <Skeleton w="20rem" h="5rem" radius="lg" />
                        <Skeleton w="20rem" h="5rem" radius="lg" />
                    </Stack>
                </Grid.Col>

                <Grid.Col xs={12} >
                    <div>
                        <Title order={2} color="dimmed">Transactions</Title>
                        <Space mb="md" />
                        {children}
                    </div>
                </Grid.Col>
            </Grid>
        </CustomPage>
    )
}