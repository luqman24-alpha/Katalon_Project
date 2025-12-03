<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>body</name>
   <tag></tag>
   <elementGuidId>1e0e9f2d-880c-4a40-81dd-5fdfa8c6bf81</elementGuidId>
   <selectorCollection>
      <entry>
         <key>CSS</key>
         <value>body</value>
      </entry>
      <entry>
         <key>XPATH</key>
         <value>//body</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <smartLocatorCollection>
      <entry>
         <key>SMART_LOCATOR</key>
         <value>internal:text=&quot;REGISTER Already have an account? Log In /html[1]/body[1]&quot;i</value>
      </entry>
   </smartLocatorCollection>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>body</value>
      <webElementGuid>da7bd0da-f7df-4a72-92df-27cae9ac4a15</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
    
        
            
            
            
                
                
                
                
                
                
                    // Get references to the password and confirm password input elements
                    const passwordInput = document.getElementById(&quot;password&quot;);
                    const confirmPasswordInput = document.getElementById(&quot;confirmpass&quot;);

                    // Add an oninput event listener to the confirm password input
                    confirmPasswordInput.addEventListener(&quot;input&quot;, function() {
                        // Get the values of both password fields
                        const passwordValue = passwordInput.value;
                        const confirmPasswordValue = confirmPasswordInput.value;

                        // Check if the passwords match
                        if (passwordValue === confirmPasswordValue) {
                            // Passwords match, clear any custom validation message
                            confirmPasswordInput.setCustomValidity('');
                        } else {
                            // Passwords do not match, set a custom validation message
                            confirmPasswordInput.setCustomValidity('Passwords do not match');
                        }
                    });
                
            
            REGISTER
            
                Already have an account? Log In
            
        
        
            
        
        
            function checkInternetStatus(callback) {
                const xhr = new XMLHttpRequest();
                xhr.open('GET', 'https://jsonplaceholder.typicode.com/posts', true);

                xhr.onload = function() {
                    if (xhr.status >= 200 &amp;&amp; xhr.status &lt; 300) {
                        callback(0); // Online
                    } else {
                        callback(1); // Offline
                    }
                };

                xhr.onerror = function() {
                    callback(1); // Offline
                };

                xhr.send();
            }

            // Example usage:
            checkInternetStatus(function(status) {
                if (status === 0) {
                    console.log(&quot;Online&quot;);
                } else {
                    console.log(&quot;Offline&quot;);
                }
            });
        
        
            function validateAlphabets(inputField) {
                // Remove any non-alphabetic characters and spaces
                inputField.value = inputField.value.replace(/[^A-Za-z\s]/g, '');
            }
            const firstname = document.getElementById('firstname');
            const lastname = document.getElementById('lastname');
            const email = document.getElementById(&quot;email&quot;);
            const password = document.getElementById(&quot;password&quot;);
            const confirmpass = document.getElementById(&quot;confirmpass&quot;);


            function createAccount(event) {
                event.preventDefault();
                checkInternetStatus(function(status) {
                    if (status === 0) {
                        let timerInterval;
                        Swal.fire({
                            title: 'Creating Account!',
                            html: 'Please wait...',
                            timerProgressBar: true,
                            didOpen: () => {
                                Swal.showLoading();
                                const b = Swal.getHtmlContainer().querySelector('b');
                                timerInterval = setInterval(() => {
                                    b.textContent = Swal.getTimerLeft();
                                }, 100);
                            },
                            willClose: () => {
                                clearInterval(timerInterval);
                            }
                        });
                        $.ajax({
                            url: &quot;/signup.php&quot;,
                            type: &quot;POST&quot;,
                            data: {
                                method: 'signin',
                                firstname: firstname.value,
                                lastname: lastname.value,
                                email: email.value,
                                password: password.value,
                                confirmpassword: confirmpass.value
                            },
                            success: function(response) {
                                Swal.close(); // Close the loading animation
                                if (response == 10) {
                                    Swal.fire({
                                        position: 'center',
                                        icon: 'error',
                                        title: 'Email Already Taken.',
                                        showConfirmButton: false,
                                        timer: 3000,
                                        allowOutsideClick: false
                                    });
                                    return;
                                } else {
                                    Swal.fire({
                                        position: 'center',
                                        icon: 'success',
                                        title: response,
                                        showConfirmButton: false,
                                        timer: 3000,
                                        allowOutsideClick: false
                                    });
                                    /*  asyn call to send mail 
                                    const xhr = new XMLHttpRequest();
                                    xhr.open('POST', 'mail.php', true);
                                    xhr.setRequestHeader('Content-Type', 'application/x-www-form-urlencoded');
    
                                    // Encode parameters and send the request
                                    const params = `firstname=${encodeURIComponent(firstname.value)}&amp;lastname=${encodeURIComponent(lastname.value)}&amp;email=${encodeURIComponent(email.value)}&amp;password=${encodeURIComponent(password.value)}`;
                                    xhr.send(params);
                                    */
                                }
                                // Redirect to signin.php after 4 seconds
                                setTimeout(function() {
                                    window.location.href = &quot;signin.php&quot;;
                                }, 1000);
                            }
                        })
                    } else {
                        Swal.fire({
                            position: 'center',
                            icon: 'error',
                            title: &quot;No Internet Connection&quot;,
                            text: &quot;Please check your internet connection and try again.&quot;,
                            showConfirmButton: false,
                            timer: 3000,
                        });
                    }
                });

            }
        
            
        function updateOnlineStatus() {
            if (navigator.onLine) {
                if (confirm('Connection restored! Please reload the page?')) {
                    location.reload();
                }
            } else {
                alert('Connection lost!');
            }
        }

        window.addEventListener('online', updateOnlineStatus);
        window.addEventListener('offline', updateOnlineStatus);

        // Initial check
        if (!navigator.onLine) {
            alert('You are currently offline!');
        }
    
    


    


/html[1]/body[1]</value>
      <webElementGuid>8f2c0b1f-c25e-4df3-9994-9191c089fe8c</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[1]/body[1]</value>
      <webElementGuid>5af6145b-a8e5-475b-98a2-61f5cc498c51</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//body</value>
      <webElementGuid>8f349dd1-ca4e-4f74-b18a-9ebf13eae11d</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//body[(text() = concat(&quot;
    
        
            
            
            
                
                
                
                
                
                
                    // Get references to the password and confirm password input elements
                    const passwordInput = document.getElementById(&quot;password&quot;);
                    const confirmPasswordInput = document.getElementById(&quot;confirmpass&quot;);

                    // Add an oninput event listener to the confirm password input
                    confirmPasswordInput.addEventListener(&quot;input&quot;, function() {
                        // Get the values of both password fields
                        const passwordValue = passwordInput.value;
                        const confirmPasswordValue = confirmPasswordInput.value;

                        // Check if the passwords match
                        if (passwordValue === confirmPasswordValue) {
                            // Passwords match, clear any custom validation message
                            confirmPasswordInput.setCustomValidity(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
                        } else {
                            // Passwords do not match, set a custom validation message
                            confirmPasswordInput.setCustomValidity(&quot; , &quot;'&quot; , &quot;Passwords do not match&quot; , &quot;'&quot; , &quot;);
                        }
                    });
                
            
            REGISTER
            
                Already have an account? Log In
            
        
        
            
        
        
            function checkInternetStatus(callback) {
                const xhr = new XMLHttpRequest();
                xhr.open(&quot; , &quot;'&quot; , &quot;GET&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;https://jsonplaceholder.typicode.com/posts&quot; , &quot;'&quot; , &quot;, true);

                xhr.onload = function() {
                    if (xhr.status >= 200 &amp;&amp; xhr.status &lt; 300) {
                        callback(0); // Online
                    } else {
                        callback(1); // Offline
                    }
                };

                xhr.onerror = function() {
                    callback(1); // Offline
                };

                xhr.send();
            }

            // Example usage:
            checkInternetStatus(function(status) {
                if (status === 0) {
                    console.log(&quot;Online&quot;);
                } else {
                    console.log(&quot;Offline&quot;);
                }
            });
        
        
            function validateAlphabets(inputField) {
                // Remove any non-alphabetic characters and spaces
                inputField.value = inputField.value.replace(/[^A-Za-z\s]/g, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
            }
            const firstname = document.getElementById(&quot; , &quot;'&quot; , &quot;firstname&quot; , &quot;'&quot; , &quot;);
            const lastname = document.getElementById(&quot; , &quot;'&quot; , &quot;lastname&quot; , &quot;'&quot; , &quot;);
            const email = document.getElementById(&quot;email&quot;);
            const password = document.getElementById(&quot;password&quot;);
            const confirmpass = document.getElementById(&quot;confirmpass&quot;);


            function createAccount(event) {
                event.preventDefault();
                checkInternetStatus(function(status) {
                    if (status === 0) {
                        let timerInterval;
                        Swal.fire({
                            title: &quot; , &quot;'&quot; , &quot;Creating Account!&quot; , &quot;'&quot; , &quot;,
                            html: &quot; , &quot;'&quot; , &quot;Please wait...&quot; , &quot;'&quot; , &quot;,
                            timerProgressBar: true,
                            didOpen: () => {
                                Swal.showLoading();
                                const b = Swal.getHtmlContainer().querySelector(&quot; , &quot;'&quot; , &quot;b&quot; , &quot;'&quot; , &quot;);
                                timerInterval = setInterval(() => {
                                    b.textContent = Swal.getTimerLeft();
                                }, 100);
                            },
                            willClose: () => {
                                clearInterval(timerInterval);
                            }
                        });
                        $.ajax({
                            url: &quot;/signup.php&quot;,
                            type: &quot;POST&quot;,
                            data: {
                                method: &quot; , &quot;'&quot; , &quot;signin&quot; , &quot;'&quot; , &quot;,
                                firstname: firstname.value,
                                lastname: lastname.value,
                                email: email.value,
                                password: password.value,
                                confirmpassword: confirmpass.value
                            },
                            success: function(response) {
                                Swal.close(); // Close the loading animation
                                if (response == 10) {
                                    Swal.fire({
                                        position: &quot; , &quot;'&quot; , &quot;center&quot; , &quot;'&quot; , &quot;,
                                        icon: &quot; , &quot;'&quot; , &quot;error&quot; , &quot;'&quot; , &quot;,
                                        title: &quot; , &quot;'&quot; , &quot;Email Already Taken.&quot; , &quot;'&quot; , &quot;,
                                        showConfirmButton: false,
                                        timer: 3000,
                                        allowOutsideClick: false
                                    });
                                    return;
                                } else {
                                    Swal.fire({
                                        position: &quot; , &quot;'&quot; , &quot;center&quot; , &quot;'&quot; , &quot;,
                                        icon: &quot; , &quot;'&quot; , &quot;success&quot; , &quot;'&quot; , &quot;,
                                        title: response,
                                        showConfirmButton: false,
                                        timer: 3000,
                                        allowOutsideClick: false
                                    });
                                    /*  asyn call to send mail 
                                    const xhr = new XMLHttpRequest();
                                    xhr.open(&quot; , &quot;'&quot; , &quot;POST&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;mail.php&quot; , &quot;'&quot; , &quot;, true);
                                    xhr.setRequestHeader(&quot; , &quot;'&quot; , &quot;Content-Type&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;application/x-www-form-urlencoded&quot; , &quot;'&quot; , &quot;);
    
                                    // Encode parameters and send the request
                                    const params = `firstname=${encodeURIComponent(firstname.value)}&amp;lastname=${encodeURIComponent(lastname.value)}&amp;email=${encodeURIComponent(email.value)}&amp;password=${encodeURIComponent(password.value)}`;
                                    xhr.send(params);
                                    */
                                }
                                // Redirect to signin.php after 4 seconds
                                setTimeout(function() {
                                    window.location.href = &quot;signin.php&quot;;
                                }, 1000);
                            }
                        })
                    } else {
                        Swal.fire({
                            position: &quot; , &quot;'&quot; , &quot;center&quot; , &quot;'&quot; , &quot;,
                            icon: &quot; , &quot;'&quot; , &quot;error&quot; , &quot;'&quot; , &quot;,
                            title: &quot;No Internet Connection&quot;,
                            text: &quot;Please check your internet connection and try again.&quot;,
                            showConfirmButton: false,
                            timer: 3000,
                        });
                    }
                });

            }
        
            
        function updateOnlineStatus() {
            if (navigator.onLine) {
                if (confirm(&quot; , &quot;'&quot; , &quot;Connection restored! Please reload the page?&quot; , &quot;'&quot; , &quot;)) {
                    location.reload();
                }
            } else {
                alert(&quot; , &quot;'&quot; , &quot;Connection lost!&quot; , &quot;'&quot; , &quot;);
            }
        }

        window.addEventListener(&quot; , &quot;'&quot; , &quot;online&quot; , &quot;'&quot; , &quot;, updateOnlineStatus);
        window.addEventListener(&quot; , &quot;'&quot; , &quot;offline&quot; , &quot;'&quot; , &quot;, updateOnlineStatus);

        // Initial check
        if (!navigator.onLine) {
            alert(&quot; , &quot;'&quot; , &quot;You are currently offline!&quot; , &quot;'&quot; , &quot;);
        }
    
    


    


/html[1]/body[1]&quot;) or . = concat(&quot;
    
        
            
            
            
                
                
                
                
                
                
                    // Get references to the password and confirm password input elements
                    const passwordInput = document.getElementById(&quot;password&quot;);
                    const confirmPasswordInput = document.getElementById(&quot;confirmpass&quot;);

                    // Add an oninput event listener to the confirm password input
                    confirmPasswordInput.addEventListener(&quot;input&quot;, function() {
                        // Get the values of both password fields
                        const passwordValue = passwordInput.value;
                        const confirmPasswordValue = confirmPasswordInput.value;

                        // Check if the passwords match
                        if (passwordValue === confirmPasswordValue) {
                            // Passwords match, clear any custom validation message
                            confirmPasswordInput.setCustomValidity(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
                        } else {
                            // Passwords do not match, set a custom validation message
                            confirmPasswordInput.setCustomValidity(&quot; , &quot;'&quot; , &quot;Passwords do not match&quot; , &quot;'&quot; , &quot;);
                        }
                    });
                
            
            REGISTER
            
                Already have an account? Log In
            
        
        
            
        
        
            function checkInternetStatus(callback) {
                const xhr = new XMLHttpRequest();
                xhr.open(&quot; , &quot;'&quot; , &quot;GET&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;https://jsonplaceholder.typicode.com/posts&quot; , &quot;'&quot; , &quot;, true);

                xhr.onload = function() {
                    if (xhr.status >= 200 &amp;&amp; xhr.status &lt; 300) {
                        callback(0); // Online
                    } else {
                        callback(1); // Offline
                    }
                };

                xhr.onerror = function() {
                    callback(1); // Offline
                };

                xhr.send();
            }

            // Example usage:
            checkInternetStatus(function(status) {
                if (status === 0) {
                    console.log(&quot;Online&quot;);
                } else {
                    console.log(&quot;Offline&quot;);
                }
            });
        
        
            function validateAlphabets(inputField) {
                // Remove any non-alphabetic characters and spaces
                inputField.value = inputField.value.replace(/[^A-Za-z\s]/g, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
            }
            const firstname = document.getElementById(&quot; , &quot;'&quot; , &quot;firstname&quot; , &quot;'&quot; , &quot;);
            const lastname = document.getElementById(&quot; , &quot;'&quot; , &quot;lastname&quot; , &quot;'&quot; , &quot;);
            const email = document.getElementById(&quot;email&quot;);
            const password = document.getElementById(&quot;password&quot;);
            const confirmpass = document.getElementById(&quot;confirmpass&quot;);


            function createAccount(event) {
                event.preventDefault();
                checkInternetStatus(function(status) {
                    if (status === 0) {
                        let timerInterval;
                        Swal.fire({
                            title: &quot; , &quot;'&quot; , &quot;Creating Account!&quot; , &quot;'&quot; , &quot;,
                            html: &quot; , &quot;'&quot; , &quot;Please wait...&quot; , &quot;'&quot; , &quot;,
                            timerProgressBar: true,
                            didOpen: () => {
                                Swal.showLoading();
                                const b = Swal.getHtmlContainer().querySelector(&quot; , &quot;'&quot; , &quot;b&quot; , &quot;'&quot; , &quot;);
                                timerInterval = setInterval(() => {
                                    b.textContent = Swal.getTimerLeft();
                                }, 100);
                            },
                            willClose: () => {
                                clearInterval(timerInterval);
                            }
                        });
                        $.ajax({
                            url: &quot;/signup.php&quot;,
                            type: &quot;POST&quot;,
                            data: {
                                method: &quot; , &quot;'&quot; , &quot;signin&quot; , &quot;'&quot; , &quot;,
                                firstname: firstname.value,
                                lastname: lastname.value,
                                email: email.value,
                                password: password.value,
                                confirmpassword: confirmpass.value
                            },
                            success: function(response) {
                                Swal.close(); // Close the loading animation
                                if (response == 10) {
                                    Swal.fire({
                                        position: &quot; , &quot;'&quot; , &quot;center&quot; , &quot;'&quot; , &quot;,
                                        icon: &quot; , &quot;'&quot; , &quot;error&quot; , &quot;'&quot; , &quot;,
                                        title: &quot; , &quot;'&quot; , &quot;Email Already Taken.&quot; , &quot;'&quot; , &quot;,
                                        showConfirmButton: false,
                                        timer: 3000,
                                        allowOutsideClick: false
                                    });
                                    return;
                                } else {
                                    Swal.fire({
                                        position: &quot; , &quot;'&quot; , &quot;center&quot; , &quot;'&quot; , &quot;,
                                        icon: &quot; , &quot;'&quot; , &quot;success&quot; , &quot;'&quot; , &quot;,
                                        title: response,
                                        showConfirmButton: false,
                                        timer: 3000,
                                        allowOutsideClick: false
                                    });
                                    /*  asyn call to send mail 
                                    const xhr = new XMLHttpRequest();
                                    xhr.open(&quot; , &quot;'&quot; , &quot;POST&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;mail.php&quot; , &quot;'&quot; , &quot;, true);
                                    xhr.setRequestHeader(&quot; , &quot;'&quot; , &quot;Content-Type&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;application/x-www-form-urlencoded&quot; , &quot;'&quot; , &quot;);
    
                                    // Encode parameters and send the request
                                    const params = `firstname=${encodeURIComponent(firstname.value)}&amp;lastname=${encodeURIComponent(lastname.value)}&amp;email=${encodeURIComponent(email.value)}&amp;password=${encodeURIComponent(password.value)}`;
                                    xhr.send(params);
                                    */
                                }
                                // Redirect to signin.php after 4 seconds
                                setTimeout(function() {
                                    window.location.href = &quot;signin.php&quot;;
                                }, 1000);
                            }
                        })
                    } else {
                        Swal.fire({
                            position: &quot; , &quot;'&quot; , &quot;center&quot; , &quot;'&quot; , &quot;,
                            icon: &quot; , &quot;'&quot; , &quot;error&quot; , &quot;'&quot; , &quot;,
                            title: &quot;No Internet Connection&quot;,
                            text: &quot;Please check your internet connection and try again.&quot;,
                            showConfirmButton: false,
                            timer: 3000,
                        });
                    }
                });

            }
        
            
        function updateOnlineStatus() {
            if (navigator.onLine) {
                if (confirm(&quot; , &quot;'&quot; , &quot;Connection restored! Please reload the page?&quot; , &quot;'&quot; , &quot;)) {
                    location.reload();
                }
            } else {
                alert(&quot; , &quot;'&quot; , &quot;Connection lost!&quot; , &quot;'&quot; , &quot;);
            }
        }

        window.addEventListener(&quot; , &quot;'&quot; , &quot;online&quot; , &quot;'&quot; , &quot;, updateOnlineStatus);
        window.addEventListener(&quot; , &quot;'&quot; , &quot;offline&quot; , &quot;'&quot; , &quot;, updateOnlineStatus);

        // Initial check
        if (!navigator.onLine) {
            alert(&quot; , &quot;'&quot; , &quot;You are currently offline!&quot; , &quot;'&quot; , &quot;);
        }
    
    


    


/html[1]/body[1]&quot;))]</value>
      <webElementGuid>3a47c6c0-470e-4674-a7ff-6c67302bea91</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
