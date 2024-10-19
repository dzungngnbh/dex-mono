// check if receipt success
export async function isSuccess(publicClient, receiptHash) {
	const tx = await publicClient.waitForTransactionReceipt({ hash: receiptHash })

	return [tx, tx.status === "success"]
}
