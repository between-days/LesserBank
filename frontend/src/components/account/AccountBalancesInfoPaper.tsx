import { getDollarTextFromCents } from "@/UIUtils"
import { Paper, Stack, Text } from "@mantine/core"

interface AccountBalancesInfoPaperProps {
    availableBalanceCents: number,
    balanceCents: number
}

export const AccountBalancesInfoPaper = ({ availableBalanceCents, balanceCents }: AccountBalancesInfoPaperProps) => {
    return <Paper withBorder shadow="sm" radius="lg" p="md" h="11rem">
        <Stack>
            <Stack spacing={0} key={0}>
                <Text size="xl" weight={500} key={0}>
                    {getDollarTextFromCents(availableBalanceCents)}
                </Text>
                <Text size="md" color="dimmed" key={1}>
                    Available Balance
                </Text>
            </Stack>
            <Stack spacing={0} key={1}>
                <Text size="xl" weight={500} key={0}>
                    {getDollarTextFromCents(balanceCents)}
                </Text>
                <Text size="md" color="dimmed" key={1}>
                    Balance
                </Text>
            </Stack>
        </Stack>
    </Paper>
}