import { ThemeIcon, Tooltip, Image } from "@mantine/core"
import { ACCOUNT_ICON_SIZE } from "./UIConstants"
import { IconCloudDollar, IconTimeDuration0 } from "@tabler/icons-react"

export function getIconForAccountType(accountType: string) {
    switch (accountType) {
        case "Savings":
            return <Tooltip label={"Savings"} position="right" transitionProps={{ duration: 0 }}>
                <ThemeIcon size={ACCOUNT_ICON_SIZE}>
                    <IconCloudDollar />
                </ThemeIcon>

            </Tooltip>
        case "Term Deposit":
            return <Tooltip label={"Term Deposit"} position="right" transitionProps={{ duration: 0 }}>
                <ThemeIcon size={ACCOUNT_ICON_SIZE}>
                    <IconTimeDuration0 />
                </ThemeIcon>

            </Tooltip>
        default:
            return <Tooltip label={"Unkown Type"} position="right" transitionProps={{ duration: 0 }}>
                <ThemeIcon size={ACCOUNT_ICON_SIZE}>
                    <IconCloudDollar />
                </ThemeIcon>
            </Tooltip>
    }
}

export function getDollarText(balanceCents: number) {
    const balanceDollars = balanceCents / 100
    return `$${balanceDollars.toLocaleString()}`
}

export function getBsbString(bsb: number) {
    const bsbString = bsb.toString()
    return bsbString.substring(0, 3) + " " + bsbString.substring(3, 6)
}

export function getAccountNumberString(accountNumber: number) {
    const anString = accountNumber.toString()
    return anString.substring(0, 3) + " " + anString.substring(3, 6) + " " + anString.substring(6, 10)
}

export function getAccountCardImage(accountType: string) {
    switch (accountType) {
        case "Savings":
            return <Image
                src="https://images.unsplash.com/photo-1579621970588-a35d0e7ab9b6?ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D&auto=format&fit=crop&w=3540&q=80"
                height={150}
                alt="Savings"
            />
        case "Term Deposit":
            return <Image
                src="https://images.unsplash.com/photo-1531161348856-92e5c7b2e6de?ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D&auto=format&fit=crop&w=3542&q=80"
                height={150}
                alt="Term Deposit"
            />
        default:
            return <Image
                src="https://images.unsplash.com/photo-1579621970588-a35d0e7ab9b6?ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D&auto=format&fit=crop&w=3540&q=80"
                height={150}
                alt="Unknown"
            />
    }
}