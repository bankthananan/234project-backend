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

WebUI.setText(findTestObject('For user role/input_username'), 'user')

WebUI.setText(findTestObject('For user role/input_password'), 'user')

WebUI.click(findTestObject('For user role/button_Login'))

WebUI.click(findTestObject('For user role/h5_Garden'))

WebUI.click(findTestObject('For user role/h6_Price is 20000.00 THB'))

WebUI.click(findTestObject('For user role/button_add to cart'))

WebUI.click(findTestObject('For user role/a_Carts            1'))

WebUI.click(findTestObject('For user role/p_Total price  20000 THB'))

WebUI.click(findTestObject('For user role/button_confirm'))

WebUI.click(findTestObject('For user role/html_ProjectBackend      chars'))

WebUI.click(findTestObject('For user role/button_Logout'))

WebUI.closeBrowser()

