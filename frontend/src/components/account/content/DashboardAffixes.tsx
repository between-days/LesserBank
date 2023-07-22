import { Affix, Button, SegmentedControl, rem } from "@mantine/core"
import { IconSquareRoundedPlus } from "@tabler/icons-react"

export const DashboardAffixes = () => {
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