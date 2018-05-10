import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.checkpoint.CheckpointFactory as CheckpointFactory
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as MobileBuiltInKeywords
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testcase.TestCaseFactory as TestCaseFactory
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testdata.TestDataFactory as TestDataFactory
import com.kms.katalon.core.testobject.ObjectRepository as ObjectRepository
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WSBuiltInKeywords
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUiBuiltInKeywords
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.navigateToUrl('http://54.191.206.59:9001/')

WebUI.setText(findTestObject('For admin role/input_username'), 'admin')

WebUI.setText(findTestObject('For admin role/input_password'), 'admin')

WebUI.click(findTestObject('For admin role/button_Login'))

WebUI.click(findTestObject('For admin role/a_Total Transaction'))

WebUI.click(findTestObject('For admin role/td_1'))

WebUI.verifyElementText(findTestObject('For admin role/td_2ndItemName'), 'Garden, Papaya')

WebUI.verifyElementText(findTestObject('For admin role/td_1stItem'), '20,120 THB')

WebUI.click(findTestObject('For admin role/td_2'))

WebUI.verifyElementText(findTestObject('For admin role/td_1stItemName'), 'Banana, Garden, Banana, Rambutan')

WebUI.verifyElementText(findTestObject('For admin role/td_2ndItem'), '60,570 THB')

WebUI.verifyElementText(findTestObject('For admin role/p_Total price'), 'Total price: 80,690 THB')

WebUI.closeBrowser()

