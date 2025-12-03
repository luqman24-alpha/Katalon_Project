import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.navigateToUrl('https://dailypakistan.com.pk/latest')

WebUI.click(findTestObject('Object Repository/Page_Latest Urdu News, Breaking News -   Da_67993e/a_Toggle navigation_dropdown-toggle menu-ur'))

WebUI.click(findTestObject('Object Repository/Page_Latest Urdu News, Breaking News -   Da_67993e/a__menu-ur'))

WebUI.click(findTestObject('Object Repository/Page_Todays Paper/div__top-center'))

WebUI.click(findTestObject('Object Repository/Page_/div__top-center'))

WebUI.click(findTestObject('Object Repository/Page_Sep 10, 2025 -  - 1, Daily Pakistan/area_Page No 8_24'))

WebUI.switchToWindowTitle('  سینیٹ ضمنی الیکشن: رانا ثنا 250ووٹ لے سینیٹر منتخب پی ٹی آئی کا بائیکاٹ')

WebUI.click(findTestObject('Object Repository/Page_250/img'))

